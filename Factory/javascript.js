// JavaScript Factory Pattern Example

class Product {
    operation() {}
}

class ConcreteProductA extends Product {
    operation() {
        return "Result of ConcreteProductA";
    }
}

class ConcreteProductB extends Product {
    operation() {
        return "Result of ConcreteProductB";
    }
}

class Creator {
    factoryMethod() {}
    someOperation() {
        const product = this.factoryMethod();
        return product.operation();
    }
}

class ConcreteCreatorA extends Creator {
    factoryMethod() {
        return new ConcreteProductA();
    }
}

class ConcreteCreatorB extends Creator {
    factoryMethod() {
        return new ConcreteProductB();
    }
}