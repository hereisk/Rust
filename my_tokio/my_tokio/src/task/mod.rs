use std::future::Future;
use std::pin::Pin;
use std::boxed::Box;
use std::task::{Context, Poll};

/* 
• The tast doesn't return anything. It is being executed for its side effects.
• dyn keyword in Box means that we will be storing different types of Futures.
• Pin insures that a value cannot be moved in the memory by placing it on the heap.
*/

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future)
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}