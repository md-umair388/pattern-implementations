<?php
// PHP Factory Pattern Example

interface Product {
    public function operation();
}

class ConcreteProductA implements Product {
    public function operation() {
        return "Result of ConcreteProductA";
    }
}

class ConcreteProductB implements Product {
    public function operation() {
        return "Result of ConcreteProductB";
    }
}

abstract class Creator {
    abstract public function factoryMethod();
    public function someOperation() {
        $product = $this->factoryMethod();
        return $product->operation();
    }
}

class ConcreteCreatorA extends Creator {
    public function factoryMethod() {
        return new ConcreteProductA();
    }
}

class ConcreteCreatorB extends Creator {
    public function factoryMethod() {
        return new ConcreteProductB();
    }
}
?>