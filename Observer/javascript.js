// JavaScript Observer Pattern Example

class Observer {
    update(message) {}
}

class ConcreteObserver extends Observer {
    update(message) {
        console.log(`Observer received: ${message}`);
    }
}

class Subject {
    constructor() {
        this.observers = [];
    }
    attach(observer) {
        this.observers.push(observer);
    }
    notify(message) {
        this.observers.forEach(observer => observer.update(message));
    }
}

// Usage:
// const subject = new Subject();
// const observer = new ConcreteObserver();
// subject.attach(observer);
// subject.notify("Hello Observers!");