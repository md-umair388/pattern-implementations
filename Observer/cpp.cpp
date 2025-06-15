// C++ Observer Pattern Example

#include <iostream>
#include <vector>
#include <string>

class Observer {
public:
    virtual void update(const std::string& message) = 0;
};

class ConcreteObserver : public Observer {
public:
    void update(const std::string& message) override {
        std::cout << "Observer received: " << message << std::endl;
    }
};

class Subject {
    std::vector<Observer*> observers;
public:
    void attach(Observer* observer) {
        observers.push_back(observer);
    }
    void notify(const std::string& message) {
        for (auto observer : observers) {
            observer->update(message);
        }
    }
};

// Usage:
// Subject subject;
// ConcreteObserver observer;
// subject.attach(&observer);
// subject.notify("Hello Observers!");