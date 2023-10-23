pub fn init_logger() {
    log4rs::init_file("wess.yaml", Default::default()).unwrap();
}

macro_rules! log_error {
    ($msg_value:expr, $status_value:expr) => {
        {
            use crate::metrics::constants::ERROR_COUNT;
            use log::error;
            use crate::errors::WessError;

            ERROR_COUNT.inc();

            let e = WessError::new($msg_value,$status_value);

            error!(target: "wess::err", "{}", e.to_string());
            e
        }
    };
}
