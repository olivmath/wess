use crate::metrics::constants::WESS_ERRORS;
use log::error;
use std::fmt::Display;

pub fn init_logger() {
    log4rs::init_file("wess.yaml", Default::default()).unwrap();
}

pub fn log_error<E: Display>(e: E) -> E {
    error!(target: "wess::err", "{e}");
    WESS_ERRORS.inc();
    e
}
