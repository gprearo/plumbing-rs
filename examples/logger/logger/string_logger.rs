use plumbing::action::handler::Handler;
use super::{LogData, LogLevel, Logger};

pub struct StringLogger {
    handler: Box<dyn Handler<LogData<String>>>
}

impl StringLogger {
    pub fn new(handler: Box<dyn Handler<LogData<String>>>) -> StringLogger {
        return StringLogger{handler};
    }
}


impl Logger<String> for StringLogger {
    fn log(&self, level: LogLevel, data: String){
        self.handler.handle(&LogData::new(level, data));
    }
} 