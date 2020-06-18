use std::rc::Rc;

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
    callback: Rc<dyn Fn(T)>,
}

pub struct Sink<T> {
    callback: Rc<dyn Fn(T)>,
}

impl<T> Sink<T> {
    
}