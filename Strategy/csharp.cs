// C# Strategy Pattern Example

public interface IStrategy
{
    int Execute(int a, int b);
}

public class ConcreteStrategyAdd : IStrategy
{
    public int Execute(int a, int b) => a + b;
}

public class ConcreteStrategySubtract : IStrategy
{
    public int Execute(int a, int b) => a - b;
}

public class Context
{
    private IStrategy strategy;
    public Context(IStrategy strategy) => this.strategy = strategy;
    public void SetStrategy(IStrategy strategy) => this.strategy = strategy;
    public int ExecuteStrategy(int a, int b) => strategy.Execute(a, b);
}

// Usage:
// var context = new Context(new ConcreteStrategyAdd());
// Console.WriteLine(context.ExecuteStrategy(5, 3)); // 8
// context.SetStrategy(new ConcreteStrategySubtract());
// Console.WriteLine(context.ExecuteStrategy(5, 3)); // 2