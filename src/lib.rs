use std::{process::Child, str::FromStr, sync::Arc};
use thirtyfour::WebDriver;
use tokio::sync::Notify;

mod utils;
pub use utils::confirm_prompt;

mod driver;
pub struct Driver {
    child: Child,
    driver: WebDriver,

    config: Config,
}

#[derive(Debug)]
pub enum Config {
    Local {
        notifier: Arc<Notify>,
        host: Option<Host>,
        port: Option<Port>,
        browser: Browser,
        profile: &'static str,
    },

    Docker {
        notifier: Arc<Notify>,
        host: Option<Host>,
        port: Option<Port>,
        browser: Browser,
        profile: &'static str,
    },
}

mod host;
#[derive(Debug)]
pub struct Host(String);

#[derive(Debug)]
pub struct Port(u16);

mod browser;
#[derive(Debug)]
pub enum Browser {
    Edge(Option<&'static str>),
    Chrome(Option<&'static str>),
    FireFox(Option<&'static str>),
}
