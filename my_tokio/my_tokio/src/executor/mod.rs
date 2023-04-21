use crate::task::Task;

use std::collections::VecDeque;
use std::task::{Context, Poll};
use std::task::{Waker, RawWaker, RawWakerVTable};

use std::future::Future;

pub struct MyTokio {
    task_queue: VecDeque<Task>
}

impl MyTokio {
    pub fn new() -> MyTokio {
        MyTokio { 
            task_queue: VecDeque::new()
        }
    }

    pub fn spawn(&mut self, task: impl Future<Output = ()> + 'static) {
        let task = Task::new(task);
        self.task_queue.push_back(task)
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {}, // task is done
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

// this doesn't actualy do anything but is used to imitate vtable and waker behaviour
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op,no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

