# Plumbing 

Plumbing is a library intended to provide some generic pieces to make the development more clean, modular and mantainable.
It's based on simple concepts, like Task, Handler, Condition, Filter, with a minimal traits but yet allows powerful implementations.

## Example

Let's suppose we wat to make a logger library to be used in a variety of scenarios.
The logger must provide us ways to simple log in several different places (like console, file, ElasticSearch).

Let's define our logger trait:

```rust
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error
}

//It uses a generics, we never know... Maybe we can have a json impl 
//or HashMap to log indexed values on ElasticSearch
pub trait Logger<T> {
    fn log(&self, level: LogLevel, data: T);
}
```

Now we can implement it with a concrete type, let's use String

```rust
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
```

What I did is to receive a Handler on construction, this handler will be responsible to actually log and it is a feature of plumbing crate.

This allow us to build this handler with a variety of tools. We can use LinearHandlers or ParallelHandler to execute more than one handler, like a handler to log on screen and other to save in a file, each one with its particularities.

That is exactly what I will do, I will make a factory for our Logger that puts two handler togheter.

```rust
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
``` 

Here I crate a FileLogger to save our entries on `log.txt`, a ConsoleLogger to print it on screen and then a ParallelHandlers, to bring those two together.

Now let's see how the main function would be.

```rust
fn main() {
    let logger = App1LoggerFactory::get_logger();
    logger.log(LogLevel::Trace, String::from("Trace example!"));
    logger.log(LogLevel::Info, String::from("Info example!"));
    logger.log(LogLevel::Warn, String::from("Warn example!"));
    logger.log(LogLevel::Error, String::from("Error example!"));
}
```

And then I have a simple logger with the flexibility to change it by simple changing the building parts.