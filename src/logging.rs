use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn setup_logging() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{l}] - {m}\n")))
        .build("/tmp/ha-events.log")
        .expect("Failed to set up log file");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Info))
        .expect("Failed to create logging configuration");

    log4rs::init_config(config)
        .expect("Failed to init log4rs");
}
