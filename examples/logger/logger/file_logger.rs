use plumbing::action::handler::Handler;
use std::fs::{OpenOptions};
use std::io::Write;
use super::{LogData, LogLevel};

pub struct FileLogger {
    path: String
}

impl FileLogger {
    pub fn new (path: String) -> FileLogger {
        return FileLogger{path};
    }
}

impl Handler<LogData<String>> for FileLogger {
    fn handle(&self, data: &LogData<String>) {
        let str_level = match data.level {
            LogLevel::Trace => "[TRACE]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]"
        };

        let mut file = OpenOptions::new()
                                          .append(true)
                                          .create(true)
                                          .open(&self.path)
                                          .expect(&format!("Cannot open file {}.", self.path));
        let log_line = format!("{} {}\n", str_level, data.data);
        let _ = file.write(&log_line.as_bytes());
    }
}