// Java Factory Pattern Example

interface Product {
    String operation();
}

class ConcreteProductA implements Product {
    public String operation() {
        return "Result of ConcreteProductA";
    }
}

class ConcreteProductB implements Product {
    public String operation() {
        return "Result of ConcreteProductB";
    }
}

abstract class Creator {
    public abstract Product factoryMethod();
    public String someOperation() {
        Product product = factoryMethod();
        return product.operation();
    }
}

class ConcreteCreatorA extends Creator {
    public Product factoryMethod() {
        return new ConcreteProductA();
    }
}

class ConcreteCreatorB extends Creator {
    public Product factoryMethod() {
        return new ConcreteProductB();
    }
}