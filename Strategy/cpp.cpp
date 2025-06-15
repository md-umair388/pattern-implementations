// C++ Strategy Pattern Example

#include <iostream>
#include <memory>

class Strategy {
public:
    virtual int execute(int a, int b) const = 0;
    virtual ~Strategy() {}
};

class ConcreteStrategyAdd : public Strategy {
public:
    int execute(int a, int b) const override {
        return a + b;
    }
};

class ConcreteStrategySubtract : public Strategy {
public:
    int execute(int a, int b) const override {
        return a - b;
    }
};

class Context {
    std::unique_ptr<Strategy> strategy;
public:
    Context(std::unique_ptr<Strategy> s) : strategy(std::move(s)) {}
    void setStrategy(std::unique_ptr<Strategy> s) {
        strategy = std::move(s);
    }
    int executeStrategy(int a, int b) const {
        return strategy->execute(a, b);
    }
};

// Usage:
// Context context(std::make_unique<ConcreteStrategyAdd>());
// std::cout << context.executeStrategy(5, 3) << std::endl; // 8
// context.setStrategy(std::make_unique<ConcreteStrategySubtract>());
// std::cout << context.executeStrategy(5, 3) << std::endl; // 2