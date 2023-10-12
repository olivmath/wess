pub fn init_logger() {
    log4rs::init_file("wess.yaml", Default::default()).unwrap();
}

macro_rules! log_error {
    ($err:expr) => {
        {
            use crate::metrics::constants::ERROR_COUNT;
            use log::error;

            ERROR_COUNT.inc();

            let e = $err;
            let e_str = e.to_string();

            error!(target: "wess::err", "{}", e_str);
            e
        }
    };
}
