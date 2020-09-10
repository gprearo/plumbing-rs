pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error
}
//Use a generics, never knows... Maybe we can have a json impl
pub trait Logger<T> {
    fn log(&self, level: LogLevel, data: T);
}
pub struct LogData<T> {
    pub level: LogLevel,
    pub data: T
}

impl<T> LogData<T> {
    pub fn new(level: LogLevel, data: T) -> LogData<T> {
        return LogData{level, data};
    }
}