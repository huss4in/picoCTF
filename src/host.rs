use std::str::FromStr;

use super::{Host, Port};

impl FromStr for Host {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl FromStr for Port {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(port) => Ok(Self(port)),
            Err(_) => Err("Invalid port"),
        }
    }
}
