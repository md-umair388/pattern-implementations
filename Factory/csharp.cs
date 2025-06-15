// C# Factory Pattern Example

public interface IProduct
{
    string Operation();
}

public class ConcreteProductA : IProduct
{
    public string Operation() => "Result of ConcreteProductA";
}

public class ConcreteProductB : IProduct
{
    public string Operation() => "Result of ConcreteProductB";
}

public abstract class Creator
{
    public abstract IProduct FactoryMethod();
    public string SomeOperation()
    {
        var product = FactoryMethod();
        return product.Operation();
    }
}

public class ConcreteCreatorA : Creator
{
    public override IProduct FactoryMethod() => new ConcreteProductA();
}

public class ConcreteCreatorB : Creator
{
    public override IProduct FactoryMethod() => new ConcreteProductB();
}