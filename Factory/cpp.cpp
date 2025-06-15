// C++ Factory Pattern Example

#include <iostream>
#include <memory>
#include <string>

class Product {
public:
    virtual std::string operation() const = 0;
    virtual ~Product() {}
};

class ConcreteProductA : public Product {
public:
    std::string operation() const override {
        return "Result of ConcreteProductA";
    }
};

class ConcreteProductB : public Product {
public:
    std::string operation() const override {
        return "Result of ConcreteProductB";
    }
};

class Creator {
public:
    virtual std::unique_ptr<Product> factoryMethod() const = 0;
    std::string someOperation() const {
        auto product = factoryMethod();
        return product->operation();
    }
};

class ConcreteCreatorA : public Creator {
public:
    std::unique_ptr<Product> factoryMethod() const override {
        return std::make_unique<ConcreteProductA>();
    }
};

class ConcreteCreatorB : public Creator {
public:
    std::unique_ptr<Product> factoryMethod() const override {
        return std::make_unique<ConcreteProductB>();
    }
};