<?php
// PHP Observer Pattern Example

interface Observer {
    public function update($message);
}

class ConcreteObserver implements Observer {
    public function update($message) {
        echo "Observer received: $message\n";
    }
}

class Subject {
    private $observers = [];
    public function attach(Observer $observer) {
        $this->observers[] = $observer;
    }
    public function notify($message) {
        foreach ($this->observers as $observer) {
            $observer->update($message);
        }
    }
}

// Usage:
// $subject = new Subject();
// $observer = new ConcreteObserver();
// $subject->attach($observer);
// $subject->notify("Hello Observers!");
?>