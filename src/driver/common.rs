use std::{process::Child, sync::Arc};

use thirtyfour::WebDriver;
use tokio::sync::Notify;

struct Driver {
    child: Child,
    client: WebDriver,
    driver_type: DriverType,
    notifier: Arc<Notify>,
}

enum DriverType {
    Docker { host: String, port: u16 },
    Edge { host: String, port: u16 },
    Chrome { host: String, port: u16 },
    Firefox { host: String, port: u16 },
}
