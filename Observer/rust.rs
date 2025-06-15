// Rust Observer Pattern Example

trait Observer {
    fn update(&self, message: &str);
}

struct ConcreteObserver;
impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("Observer received: {}", message);
    }
}

struct Subject<'a> {
    observers: Vec<&'a dyn Observer>,
}

impl<'a> Subject<'a> {
    fn new() -> Self {
        Self { observers: Vec::new() }
    }
    fn attach(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }
    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

// Usage:
// let mut subject = Subject::new();
// let observer = ConcreteObserver;
// subject.attach(&observer);
// subject.notify("Hello Observers!");