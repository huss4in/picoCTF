use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    println!();
    let notify = Arc::new(Notify::new());
    let notifier = notify.clone();

    ctrlc::set_handler(move || {
        println!("\rCtrl+C!");
        notifier.notify_one();
    })
    .unwrap();

    run(notify.clone()).await;
}

async fn run(notifier: Arc<Notify>) {
    let config = picoCTF::Config::Local {
        notifier,
        host: "localhost".parse().ok(),
        port: "3000".parse().ok(),
        browser: picoCTF::Browser::Chrome(None),
        profile: "",
    };

    let mut driver = picoCTF::Driver::new(config).await;

    driver.run().await;

    driver.quit().await;
}
