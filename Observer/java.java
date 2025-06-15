// Java Observer Pattern Example

import java.util.*;

interface Observer {
    void update(String message);
}

class ConcreteObserver implements Observer {
    public void update(String message) {
        System.out.println("Observer received: " + message);
    }
}

class Subject {
    private List<Observer> observers = new ArrayList<>();
    public void attach(Observer observer) {
        observers.add(observer);
    }
    public void notifyObservers(String message) {
        for (Observer observer : observers) {
            observer.update(message);
        }
    }
}

// Usage:
// Subject subject = new Subject();
// Observer observer = new ConcreteObserver();
// subject.attach(observer);
// subject.notifyObservers("Hello Observers!");