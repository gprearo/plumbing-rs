mod logger;
use logger::{App1LoggerFactory, LogLevel};
fn main() {
    let logger = App1LoggerFactory::get_logger();
    logger.log(LogLevel::Trace, String::from("Trace example!"));
    logger.log(LogLevel::Info, String::from("Info example!"));
    logger.log(LogLevel::Warn, String::from("Warn example!"));
    logger.log(LogLevel::Error, String::from("Error example!"));
}