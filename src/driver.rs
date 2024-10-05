use std::{
    collections::HashSet,
    env,
    os::unix::process::CommandExt,
    process::{Child, Command, Stdio},
    sync::Arc,
    time::Duration,
};
use thirtyfour::{error::WebDriverResult, DesiredCapabilities, WebDriver};
use tokio::sync::Notify;

pub struct Driver {
    child: Child,
    driver: WebDriver,
    notifier: Arc<Notify>,
    driver_name: String,
}

impl Driver {
    pub async fn init(notifier: Arc<Notify>) {
        let mut driver = Self::new(notifier).await;
        driver.run().await;
        driver.quit().await;
    }

    pub async fn new(notifier: Arc<Notify>) -> Self {
        if let Some((mut child, DriverType(driver_name, port))) = DriverType::new() {
            log::info!("Starting {}:{}... ⌛", &driver_name, port);

            match child.stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
                Ok(mut child) => {
                    for counter in 0..=10 {
                        log::debug!("Waiting 1s... ⌛");

                        tokio::select! {
                            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                            }
                            _ = notifier.notified() => {
                                child.kill().unwrap_or_else(|e| log::error!("Failed to kill {} ❌: {}", &driver_name, e));
                                log::warn!("Shutting {} down... ✅", &driver_name);

                                if driver_name == "docker" {
                                    if let Err(e) = Command::new("docker")
                                        .args(["stop", "selenium"])
                                        .stdout(Stdio::null())
                                        .stderr(Stdio::null())
                                        .spawn()
                                    {
                                        log::error!("Failed to stop selenium ❌: {}", e)
                                    }
                                }

                                break;
                            }
                        }

                        let url = format!("http://localhost:{}", port);

                        if let Ok(driver) =
                            WebDriver::new(&url, DesiredCapabilities::chrome()).await
                        {
                            log::warn!("Started {} {} ✅", &driver_name, &url);
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            return Self {
                                child,
                                driver,
                                notifier,
                                driver_name,
                            };
                        }

                        if counter > 10 {
                            log::error!("Failed to start {} after 10s ❌", &driver_name);
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to start {} ❌: {}", &driver_name, e);
                }
            }
        };

        std::process::exit(1);
    }

    pub async fn quit(mut self) {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(100)) => {
            }
            _ = self.notifier.notified() => {
                log::warn!("Shutting {} down... ⌛", self.driver_name);
            }
        };

        if let Err(e) = self.driver.quit().await {
            log::error!("Failed to quit WebDriver ❌: {}", e);
        }

        if let Err(e) = self.child.kill() {
            log::error!("Failed to kill {} ❌: {}", self.driver_name, e);
        }

        if self.driver_name == "docker" {
            if let Err(e) = Command::new("docker")
                .args(["stop", "selenium"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                log::error!("Failed to stop selenium ❌: {}", e)
            }
        }

        log::warn!("Shutting {} down... ✅", self.driver_name);
    }

    pub async fn run(&self) {
        log::info!("Navigating to 'http://google.com'... ⌛");

        self.driver.goto("http://google.com").await;
    }
}

struct DriverType(String, u16);

impl DriverType {
    fn new() -> Option<(Command, Self)> {
        static VALID_DRIVERS: &[&str] = &["docker", "chromedriver", "geckodriver", "edgedriver"];

        match (
            env::var("DRIVER_TYPE"),
            env::var("DRIVER_PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok()),
        ) {
            (Ok(driver), Some(port @ 1024..=65535)) if VALID_DRIVERS.contains(&driver.as_str()) => {
                let mut child = Command::new(&driver);

                if driver == "docker" {
                    child.args([
                        "run",
                        "--rm",
                        "--name",
                        "selenium",
                        "--shm-size=2g",
                        "-p",
                        &format!("{}:4444", port),
                        "-e",
                        "SE_VNC_NO_PASSWORD=1",
                        "selenium/standalone-chrome",
                    ]);
                } else {
                    child.arg(format!("--port={}", port));
                }

                return Some((child, Self(driver, port)));
            }
            (Ok(driver), Some(port @ 1024..=65535)) => {
                log::error!(
                    "DRIVER_TYPE {} is not supported\nUse ('{}')",
                    driver,
                    VALID_DRIVERS.join("' | '")
                );
            }
            (_, Some(port)) => {
                log::error!("PORT {} should be between 1024 and 65535", port)
            }
            _ => log::error!("DRIVER_TYPE & DRIVER_PORT are missing"),
        };

        None
    }
}
