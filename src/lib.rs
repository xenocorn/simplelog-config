use simplelog;
use serde::Deserialize;
use std::fs::OpenOptions;
use log::SetLoggerError;

const FORMAT: &str = "[%d.%m.%Y %H:%M:%S]";

#[derive(Deserialize, Debug)]
pub struct LogConfig{
    pub term_logger: Option<TermLogConfig>,
    pub write_logger: Option<WriteLogConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TermLogConfig{
    pub log_level: LogLevel,
}

#[derive(Deserialize, Debug)]
pub struct WriteLogConfig{
    pub log_level: LogLevel,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub enum LogLevel{
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel{
    pub fn to_level_filter(&self) -> simplelog::LevelFilter{
        match self{
            LogLevel::Off => {simplelog::LevelFilter::Off}
            LogLevel::Error => {simplelog::LevelFilter::Error}
            LogLevel::Warn => {simplelog::LevelFilter::Warn}
            LogLevel::Info => {simplelog::LevelFilter::Info}
            LogLevel::Debug => {simplelog::LevelFilter::Debug}
            LogLevel::Trace => {simplelog::LevelFilter::Trace}
        }
    }
}

pub fn setup_logger(config: &Vec<LogConfig>) -> Result<(), SetLoggerError> {
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();
    for log_conf in config{
        if let Some(term_logger_conf) = &log_conf.term_logger{
            let conf = simplelog::ConfigBuilder::new().set_time_format_str(FORMAT).build();
            loggers.push(simplelog::TermLogger::new(term_logger_conf.log_level.to_level_filter(),
                                                    conf,
                                                    simplelog::TerminalMode::Mixed,
                                                    simplelog::ColorChoice::Auto));
        }
        if let Some(write_logger_conf) = &log_conf.write_logger{
            let conf = simplelog::ConfigBuilder::new().set_time_format_str(FORMAT).build();
            let file = OpenOptions::new().write(true).append(true).create(true).open(write_logger_conf.path.clone()).expect("Cannot open log file");
            loggers.push(simplelog::WriteLogger::new(write_logger_conf.log_level.to_level_filter(),
                                                     conf,
                                                     file));
        }
    }
    simplelog::CombinedLogger::init(loggers)
}
