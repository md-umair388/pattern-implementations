# Python Observer Pattern Example

class Observer:
    def update(self, message):
        pass

class ConcreteObserver(Observer):
    def update(self, message):
        print(f"Observer received: {message}")

class Subject:
    def __init__(self):
        self._observers = []

    def attach(self, observer):
        self._observers.append(observer)

    def notify(self, message):
        for observer in self._observers:
            observer.update(message)

# Usage
# subject = Subject()
# observer = ConcreteObserver()
# subject.attach(observer)
# subject.notify("Hello Observers!")