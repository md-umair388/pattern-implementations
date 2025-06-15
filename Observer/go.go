// Go Observer Pattern Example

package observer

import "fmt"

type Observer interface {
    Update(message string)
}

type ConcreteObserver struct{}

func (co *ConcreteObserver) Update(message string) {
    fmt.Println("Observer received:", message)
}

type Subject struct {
    observers []Observer
}

func (s *Subject) Attach(observer Observer) {
    s.observers = append(s.observers, observer)
}

func (s *Subject) Notify(message string) {
    for _, o := range s.observers {
        o.Update(message)
    }
}

// Usage:
// subject := &Subject{}
// observer := &ConcreteObserver{}
// subject.Attach(observer)
// subject.Notify("Hello Observers!")