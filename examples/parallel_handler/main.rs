
use plumbing::action::handler::{ParallelHandlers, Handler};


pub struct PrintHandler {
    id: String
}

impl PrintHandler {
    pub fn new(id: String) -> PrintHandler{
        return PrintHandler{id};
    }
}

impl Handler<String> for PrintHandler {
    fn handle(&self, data: &String) {
        print!("{} - {}\n", self.id, data);
    }
}

fn main() {
    const N: usize = 20;
    let mut handlers: Vec<Box<dyn Handler<String> + Sync>> = Vec::with_capacity(N);
    for i in 0..N {
        handlers.push(Box::new(PrintHandler::new(i.to_string())));
    }

    let parallel_handler: ParallelHandlers<String> = ParallelHandlers::new(handlers);
    parallel_handler.handle(&String::from("Hello"));
}