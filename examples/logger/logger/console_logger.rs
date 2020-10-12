use plumbing::action::handler::Handler;
use super::{LogData, LogLevel};
use paris::Logger;

pub struct ConsoleLogger {
}

impl ConsoleLogger {
    pub fn new () -> ConsoleLogger {
        return ConsoleLogger{};
    }
}

impl Handler<LogData<String>> for ConsoleLogger {
    fn handle(&self, data: &LogData<String>) {
        let mut logger = Logger::new();
        match data.level {
            LogLevel::Trace => logger.log(&format!("> {}", &data.data)),
            LogLevel::Info => logger.info(&data.data),
            LogLevel::Warn => logger.warn(&data.data),
            LogLevel::Error => logger.error(&data.data)
        };
    }
}