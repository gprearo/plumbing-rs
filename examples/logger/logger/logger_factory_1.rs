use super::{Logger, FileLogger, ConsoleLogger, LogData, StringLogger};
use plumbing::action::handler::{ParallelHandlers, Handler};
pub struct App1LoggerFactory {}

impl App1LoggerFactory {
    pub fn get_logger() -> Box<dyn Logger<String>> {
        type LogHandler = Box<dyn Handler<LogData<String>> + Sync>;
        let file_handler: LogHandler = Box::new(FileLogger::new(String::from("log.txt")));
        let console_handler: LogHandler = Box::new(ConsoleLogger::new());
        let parallel_handlers: LogHandler = Box::new(ParallelHandlers::new(Vec::from([file_handler, console_handler])));
        return Box::new(StringLogger::new(parallel_handlers));
    }
}