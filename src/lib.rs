use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};
use thirtyfour::{error::WebDriverResult, DesiredCapabilities, WebDriver};

pub struct PicoCTF {
    child: Child,
    driver: WebDriver,
}

impl PicoCTF {
    pub async fn new() -> Self {
        let child = Command::new("chromedriver")
            .arg("--port=9515")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start ChromeDriver");

        tokio::time::sleep(Duration::from_secs(2)).await;

        let driver = WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome())
            .await
            .expect("Failed to start WebDriver");

        Self { child, driver }
    }

    pub async fn run(&self) -> WebDriverResult<()> {
        println!("Navigating to http://google.com");

        self.driver.goto("http://google.com").await?;

        tokio::time::sleep(Duration::from_secs(100)).await;

        Ok(())
    }

    pub async fn quit(mut self) {
        self.driver.quit().await.expect("Failed to quit WebDriver");
        self.child.kill().expect("Failed to kill ChromeDriver");
    }
}
