// Go Factory Pattern Example

package factory

import "fmt"

type Product interface {
    Operation() string
}

type ConcreteProductA struct{}
func (ConcreteProductA) Operation() string { return "Result of ConcreteProductA" }

type ConcreteProductB struct{}
func (ConcreteProductB) Operation() string { return "Result of ConcreteProductB" }

type Creator interface {
    FactoryMethod() Product
    SomeOperation() string
}

type ConcreteCreatorA struct{}
func (ConcreteCreatorA) FactoryMethod() Product { return ConcreteProductA{} }
func (c ConcreteCreatorA) SomeOperation() string {
    return c.FactoryMethod().Operation()
}

type ConcreteCreatorB struct{}
func (ConcreteCreatorB) FactoryMethod() Product { return ConcreteProductB{} }
func (c ConcreteCreatorB) SomeOperation() string {
    return c.FactoryMethod().Operation()
}