use crate::metrics::constants::ERROR_COUNT;
use log::error;
use std::fmt::Display;

pub fn init_logger() {
    log4rs::init_file("wess.yaml", Default::default()).unwrap();
}

pub fn log_error<E: Display>(e: E) -> E {
    error!(target: "wess::err", "{e}");
    ERROR_COUNT.inc();
    e
}
