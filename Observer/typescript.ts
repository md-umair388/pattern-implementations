// TypeScript Observer Pattern Example

interface Observer {
    update(message: string): void;
}

class ConcreteObserver implements Observer {
    update(message: string): void {
        console.log(`Observer received: ${message}`);
    }
}

class Subject {
    private observers: Observer[] = [];
    attach(observer: Observer) {
        this.observers.push(observer);
    }
    notify(message: string) {
        this.observers.forEach(observer => observer.update(message));
    }
}

// Usage:
// const subject = new Subject();
// const observer = new ConcreteObserver();
// subject.attach(observer);
// subject.notify("Hello Observers!");