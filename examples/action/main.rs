use plumbing::action::{Handler, Task};

pub struct PrintHandler {}

impl PrintHandler {
    pub fn new() -> PrintHandler {
        return PrintHandler{};
    }
}

impl Handler<String> for PrintHandler {
    fn handle(&self, data: String) {
        print!("{}", data);
    }
}

pub struct StaticPrintTask {
    text: String
}

impl StaticPrintTask {
    pub fn new(text: String) -> StaticPrintTask {
        return StaticPrintTask{text};
    }
}

impl Task for StaticPrintTask {
    fn execute(&self) {
        print!("{}", self.text);
    }
}

fn main() {
    let printer = PrintHandler::new();
    let printer_task = StaticPrintTask::new(String::from("Hello, world from task\n"));
    printer.handle(String::from("Hello, world!\n"));
    printer_task.execute();
}