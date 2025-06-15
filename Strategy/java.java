// Java Strategy Pattern Example

interface Strategy {
    int execute(int a, int b);
}

class ConcreteStrategyAdd implements Strategy {
    public int execute(int a, int b) {
        return a + b;
    }
}

class ConcreteStrategySubtract implements Strategy {
    public int execute(int a, int b) {
        return a - b;
    }
}

class Context {
    private Strategy strategy;
    public Context(Strategy strategy) {
        this.strategy = strategy;
    }
    public void setStrategy(Strategy strategy) {
        this.strategy = strategy;
    }
    public int executeStrategy(int a, int b) {
        return strategy.execute(a, b);
    }
}

// Usage:
// Context context = new Context(new ConcreteStrategyAdd());
// System.out.println(context.executeStrategy(5, 3)); // 8
// context.setStrategy(new ConcreteStrategySubtract());
// System.out.println(context.executeStrategy(5, 3)); // 2