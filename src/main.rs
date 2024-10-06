use picoCTF::Driver;
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

    Driver::init(notify.clone()).await;
    // Driver::docker::init(notify.clone()).await;
    // Driver::edge::init(notify.clone()).await;
    // Driver::chrome::init(notify.clone()).await;
    // Driver::firefox::init(notify.clone()).await;
}
