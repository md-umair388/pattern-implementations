// Rust Strategy Pattern Example

trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct AddStrategy;
impl Strategy for AddStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct SubtractStrategy;
impl Strategy for SubtractStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

struct Context<T: Strategy> {
    strategy: T,
}

impl<T: Strategy> Context<T> {
    fn new(strategy: T) -> Self {
        Self { strategy }
    }
    fn execute_strategy(&self, a: i32, b: i32) -> i32 {
        self.strategy.execute(a, b)
    }
}

// Usage example:
// let context = Context::new(AddStrategy);
// println!("{}", context.execute_strategy(5, 3)); // 8