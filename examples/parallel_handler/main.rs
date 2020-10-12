
use plumbing::action::handler::{ParallelHandlers, Handler};


pub struct PrintHandler {
    id: i32
}

impl PrintHandler {
    pub fn new(id: i32) -> PrintHandler{
        return PrintHandler{id};
    }
}

impl Handler<String> for PrintHandler {
    fn handle(&self, data: &String) {
        print!("Thread ID: {:02} - {}\n", self.id, data);
    }
}

fn main() {
    const N: usize = 20;
    let mut handlers: Vec<Box<dyn Handler<String> + Sync>> = Vec::with_capacity(N);
    for i in 0..N {
        handlers.push(Box::new(PrintHandler::new(i as i32)));
    }

    let parallel_handler: ParallelHandlers<String> = ParallelHandlers::new(handlers);
    parallel_handler.handle(&String::from("Hello"));
}