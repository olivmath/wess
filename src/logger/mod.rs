use log::error;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::fmt::Display;

pub fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .target(Target::Stdout)
        .build();

    let wess_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build("log/wess.log")
        .unwrap();

    let tx_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build("log/tx.log")
        .unwrap();

    let run_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build("log/run.log")
        .unwrap();

    let err_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build("log/err.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("wess_log", Box::new(wess_log)))
        .appender(Appender::builder().build("tx_log", Box::new(tx_log)))
        .appender(Appender::builder().build("run_log", Box::new(run_log)))
        .appender(Appender::builder().build("err_log", Box::new(err_log)))
        .logger(
            Logger::builder()
                .appender("wess_log")
                .appender("stdout")
                .build("wess", log::LevelFilter::Trace),
        )
        .logger(
            Logger::builder()
                .appender("tx_log")
                .build("tx", log::LevelFilter::Trace),
        )
        .logger(
            Logger::builder()
                .appender("run_log")
                .build("run", log::LevelFilter::Trace),
        )
        .logger(
            Logger::builder()
                .appender("err_log")
                .build("err", log::LevelFilter::Error),
        )
        .build(
            Root::builder()
                .appender("tx_log")
                .appender("run_log")
                .appender("err_log")
                .build(log::LevelFilter::Off),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

pub fn log_error<E: Display>(e: E) -> E {
    error!(target: "err", "{e}");
    e
}
