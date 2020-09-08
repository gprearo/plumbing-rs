
use rayon::prelude::*;
use std::marker::Sync;
use crate::action::handler::Handler;

pub struct ParallelHandlers<T> {
    handlers: Vec<Box<dyn Handler<T> + Sync>>
}

impl<T> ParallelHandlers<T> {
    pub fn new(handlers: Vec<Box<dyn Handler<T> + Sync>>) -> ParallelHandlers<T>{
        return ParallelHandlers{handlers};
    }
}

impl<T> Handler<T> for ParallelHandlers<T> 
    where T: Sync{
    fn handle(&self, data: & T) {
        self.handlers.par_iter().for_each(| handler| handler.handle(data));
    }
}