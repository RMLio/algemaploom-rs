use std::env;

use log::{debug, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;

pub fn init_logger(debug_enabled: bool) -> anyhow::Result<()> {
    let mut log_config_file = env::current_dir()?;
    log_config_file.push("log4rs.yaml");

    if log_config_file.exists() {
        log4rs::init_file(log_config_file, Default::default())?;
    } else {
        log4rs::init_config(build_log_config(debug_enabled))?;
        debug!("Using fallback default logger config");
    }

    Ok(())
}

pub fn build_log_config(debug_enabled: bool) -> Config {
    let mut console_threshold = ThresholdFilter::new(LevelFilter::Info);
    if debug_enabled {
        console_threshold = ThresholdFilter::new(LevelFilter::Debug);
    }

    let console_appender = Appender::builder().filter(Box::new(console_threshold)).build(
        "console",
        Box::new(
        ConsoleAppender::builder()
            .target(Target::Stderr)
            .encoder(Box::new(PatternEncoder::new(

                "{h({d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {M}:{L} - {([{l}]:):<8} {m})}{n}",
            )))
            .build()),
    );

    let mut appenders = Vec::new();
    appenders.push(console_appender);
    let mut root_builder = Root::builder();
    root_builder = root_builder.appender("console");

    if debug_enabled {
        let debug_threshold = ThresholdFilter::new(LevelFilter::Trace);

        let file_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {M}:{L} - {([{l}]:):<8} {m}{n}",
            )))
            .append(false)
            .build(format!(
                "{}/log/execution.log",
                env::current_dir().unwrap().to_string_lossy()
            ))
            .unwrap();

        let file_sink = Appender::builder()
            .filter(Box::new(debug_threshold))
            .build("file", Box::new(file_appender));

        appenders.push(file_sink);
        root_builder = root_builder.appender("file");
    }

    Config::builder()
        .appenders(appenders)
        .build(root_builder.build(LevelFilter::Trace))
        .unwrap()
}
