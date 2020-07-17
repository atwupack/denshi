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
    cbs: Rc<RefCell<Vec<Box< dyn Fn(&T)>>>>,
}

impl<E: 'static> Event<E> {
    pub fn new() -> Self {
        Event {
            cbs: Default::default(),
        }
    }

    pub fn new_with_sink() -> (Self, Sink<E>) {
        let event = Event::new();
        let sink = Sink::from_fn(|e| {
            event.send(e)
        });
        (event, sink)
    }

    fn observe(&self, f: impl Fn(&E) + 'static) {
        self.cbs.borrow_mut().push(Box::new(f))
    }

    fn send(&self, event: &E) {
        for cb in &*self.cbs.borrow() {
            cb(event)
        }
    }
}

#[derive(Clone)]
pub struct Sink<T> {
    call: Rc<dyn Fn(&T)>,
}

impl<T> Sink<T> {
    pub fn from_fn(f: impl Fn(&T) + 'static) -> Self {
        Sink {
            call: Rc::new(f),
        }
    }

    pub fn send(&self, value: &T) {
        (self.call)(value)
    }
}