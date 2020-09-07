use crate::action::handler::Handler;

pub struct LinearHandlers<T> {
    handlers: Vec<Box<dyn Handler<T>>>
}

impl<T> LinearHandlers<T> {
    pub fn new(handlers: Vec<Box<dyn Handler<T>>>) -> LinearHandlers<T>{
        return LinearHandlers{handlers};
    }
}

impl<T> Handler<T> for LinearHandlers<T> {
    fn handle(&self, data: & T) {
        self.handlers.iter().for_each(| handler| handler.handle(data))
    }
}