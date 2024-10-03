use picoCTF::PicoCTF;
use std::sync::Arc;
use thirtyfour::prelude::*;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let notify = Arc::new(Notify::new());
    let notifier = notify.clone();

    ctrlc::set_handler(move || {
        println!("\rCtrl+C!");
        notifier.notify_one();
    })
    .unwrap();

    let picoCTF = PicoCTF::new().await;

    tokio::select! {
        _ = picoCTF.run() => {
            println!("Done!");
        }
        _ = notify.notified() => {
            println!("Shutting down...");
        }
    }

    picoCTF.quit().await;

    Ok(())
}
