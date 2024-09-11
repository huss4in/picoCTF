use std::{sync::Arc, time::Duration};
use thirtyfour::prelude::*;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver = WebDriver::new("http://localhost:4444", DesiredCapabilities::chrome()).await?;

    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\rCtrl+C!");
        // Notify the async function to shut down
        notify_clone.notify_one();
    })
    .expect("Error setting Ctrl-C handler");

    println!("Navigating to http://google.com");

    driver.goto("http://google.com").await?;

    // Use tokio::select! to wait for either the sleep or the notify
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(100)) => {
            // Sleep completed without interruption
        }
        _ = notify.notified() => {
            // Ctrl+C received
            println!("Shutting down...");
        }
    }

    driver.quit().await?;

    Ok(())
}
