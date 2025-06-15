// JavaScript Strategy Pattern Example

class Strategy {
    execute(a, b) {}
}

class ConcreteStrategyAdd extends Strategy {
    execute(a, b) {
        return a + b;
    }
}

class ConcreteStrategySubtract extends Strategy {
    execute(a, b) {
        return a - b;
    }
}

class Context {
    constructor(strategy) {
        this.strategy = strategy;
    }
    setStrategy(strategy) {
        this.strategy = strategy;
    }
    executeStrategy(a, b) {
        return this.strategy.execute(a, b);
    }
}

// Usage:
// let context = new Context(new ConcreteStrategyAdd());
// console.log(context.executeStrategy(5, 3)); // 8
// context.setStrategy(new ConcreteStrategySubtract());
// console.log(context.executeStrategy(5, 3)); // 2