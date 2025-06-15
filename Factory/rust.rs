// Rust Factory Pattern Example

trait Product {
    fn operation(&self) -> String;
}

struct ConcreteProductA;
impl Product for ConcreteProductA {
    fn operation(&self) -> String {
        "Result of ConcreteProductA".to_string()
    }
}

struct ConcreteProductB;
impl Product for ConcreteProductB {
    fn operation(&self) -> String {
        "Result of ConcreteProductB".to_string()
    }
}

trait Creator {
    fn factory_method(&self) -> Box<dyn Product>;
    fn some_operation(&self) -> String {
        self.factory_method().operation()
    }
}

struct ConcreteCreatorA;
impl Creator for ConcreteCreatorA {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductA)
    }
}

struct ConcreteCreatorB;
impl Creator for ConcreteCreatorB {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductB)
    }
}