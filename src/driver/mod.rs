mod docker;
mod common;

use std::{
    collections::HashSet,
    env,
    process::{Child, Command, Stdio},
    sync::Arc,
    time::Duration,
};
use thirtyfour::{error::WebDriverResult, DesiredCapabilities, WebDriver};
use tokio::sync::Notify;

use crate::utils;

pub struct Driver {
    child: Child,
    driver: WebDriver,
    notifier: Arc<Notify>,
    driver_type: DriverType,
}

impl Driver {
    pub async fn init(notifier: Arc<Notify>) {
        let mut driver = Self::new(notifier).await;
        driver.run().await;
        driver.quit().await;
    }

    async fn new(notifier: Arc<Notify>) -> Self {
        if let Some((mut child, driver_type)) = DriverType::new() {
            const SEC: usize = 10;

            log::warn!("Starting {}:{}... ⌛", &driver_type.name, driver_type.port);
            match child
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(mut child) => {
                    let url = format!("http://{}:{}", &driver_type.host, driver_type.port);

                    for counter in 0..SEC {
                        log::debug!("Waiting {}/{}s... ⌛", counter + 1, SEC);
                        tokio::time::sleep(Duration::from_secs(1)).await;

                        let mut caps = DesiredCapabilities::chrome();
                        tokio::select! {
                            driver = WebDriver::new(&url, caps) => {
                                match driver {
                                    Ok(driver) => {
                                        log::info!("Starting {} {} ✅", &driver_type.name, url);
                                        tokio::time::sleep(Duration::from_secs(2)).await;

                                        return Self {
                                            child,
                                            driver,
                                            notifier,
                                            driver_type,
                                        };
                                    }
                                    Err(e) => {
                                        if counter == SEC {
                                            log::error!(
                                                "Failed to conntect to {} after {}s ❌:\n{}\n",
                                                driver_type.name,
                                                SEC,
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                                continue;
                            }
                            _ = notifier.notified() => {
                                break;
                            }
                        }
                    }
                    Self::kill(&mut child, driver_type.name).await;
                }
                Err(e) => {
                    log::error!("Failed to start {} ❌\n{}", driver_type.name, e);
                }
            }
        };

        std::process::exit(1);
    }

    async fn run(&self) {
        log::warn!("Window focus... ⌛");
        self.driver.execute("window.focus();", vec![]).await;
        log::debug!("Window focus... ✅");

        log::warn!("Navigating to 'http://google.com'... ⌛");
        self.driver.goto("http://google.com").await;
        log::debug!("Navigating to 'http://google.com'... ✅");

        log::warn!("Maximizing window... ⌛");
        self.driver.maximize_window().await;
        log::debug!("Maximizing window... ✅");
    }

    async fn quit(mut self) {
        const TIMEOUT: Duration = Duration::from_secs(100);

        log::warn!("Quitting WebDriver in {}s... ⌛", TIMEOUT.as_secs());

        tokio::select! {
            _ = tokio::time::sleep(TIMEOUT) => {
                log::warn!("Automatic shutdown ({}s)... ⌛", TIMEOUT.as_secs());
            }
            _ = self.notifier.notified() => {
                log::warn!("Shutting {} down... ⌛", self.driver_type.name);
            }
        }

        tokio::select! {
            err = self.driver.quit() => {
                match err {
                    Ok(_) => {
                        log::info!("WebDriver has been closed ✅");
                    }
                    Err(e) => {
                        log::error!("Failed to quit WebDriver ❌: {}", e);
                    }
                }
            }

            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                log::warn!("Forcing shutdown... ⌛");
            }

            _ = self.notifier.notified() => {
                log::warn!("Shutting {} down... ⌛", self.driver_type.name);
            }
        }

        Self::kill(&mut self.child, self.driver_type.name).await;
    }

    async fn kill(child: &mut Child, name: impl Into<String>) {
        let name = name.into();

        if let Err(e) = child.kill() {
            log::error!("Failed to kill {} ❌: {}", name, e);
        }

        if name == "docker" {
            if let Err(e) = Command::new("docker")
                .args(["stop", "selenium"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                log::error!("Failed to stop selenium ❌: {}", e)
            }
        }

        log::info!("Shutting {} down... ✅", name);
    }
}

struct DriverType {
    name: String,
    host: String,
    port: u16,
}

impl DriverType {
    fn new() -> Option<(Command, Self)> {
        let port_range = 1024..=65535;

        match (
            env::var("DRIVER_TYPE"),
            env::var("DRIVER_HOST"),
            env::var("DRIVER_ARGS"),
            env::var("DRIVER_PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok()),
        ) {
            (Ok(name), host, args, Some(port @ PORT_RANGE)) => {
                let mut host = host.unwrap_or("localhost".into());
                let mut child = Command::new(&name);

                if name == "docker" {
                    host = "localhost".into();
                    child
                        .args(["run", "--rm", "--shm-size=2g"])
                        .args(["--name", "selenium"])
                        .args(["-p", &format!("{}:4444", port)])
                        .args(["-e", "SE_VNC_NO_PASSWORD=1"])
                        .args(["-v", "./user-data:/home/seluser/.config/google-chrome"])
                        .arg("selenium/standalone-chrome");
                } else {
                    if let Ok(args) = args {
                        child.args(args.split_whitespace());
                    }
                    child.arg(format!("--port={}", port));
                }

                return Some((child, Self { name, host, port }));
            }
            (_, _, _, Some(port)) => {
                log::error!(
                    "PORT {} should be between {} and {}",
                    port,
                    port_range.start(),
                    port_range.end()
                );
            }
            _ => log::error!("DRIVER_TYPE & DRIVER_PORT are missing"),
        };

        None
    }
}
