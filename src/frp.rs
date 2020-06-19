use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Behavior<T> {
    call: Rc<dyn Fn() -> T>,
}

impl<T> Behavior<T> {
    pub fn from_fn(f: impl Fn() -> T + 'static) -> Self {
        Behavior {
            call: Rc::new(f),
        }
    }

    pub fn value(&self) -> T {
        (self.call)()
    }
}

#[derive(Clone)]
pub struct Event<T> {
    callbacks: Rc<  dyn Fn(T)>,
}

pub struct Sink<T> {
    callbacks: Rc<RefCell<Vec<Box<dyn Fn(&T)>>>>,
}

impl<T> Sink<T> {

    pub fn new() -> Self {
        Sink {
            callbacks: Default::default(),
        }
    }

    pub fn from_fn(f: impl Fn(&T) + 'static) -> Self {
        Sink {

        }
    }

    fn send(&self, value: &T) {
        let cbs = self.callbacks.borrow();
        for cb in &*cbs {
            cb(value);
        }
    }
}