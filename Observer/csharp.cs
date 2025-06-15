// C# Observer Pattern Example

using System;
using System.Collections.Generic;

public interface IObserver
{
    void Update(string message);
}

public class ConcreteObserver : IObserver
{
    public void Update(string message)
    {
        Console.WriteLine($"Observer received: {message}");
    }
}

public class Subject
{
    private List<IObserver> observers = new List<IObserver>();
    public void Attach(IObserver observer)
    {
        observers.Add(observer);
    }
    public void Notify(string message)
    {
        foreach (var observer in observers)
        {
            observer.Update(message);
        }
    }
}

// Usage:
// var subject = new Subject();
// var observer = new ConcreteObserver();
// subject.Attach(observer);
// subject.Notify("Hello Observers!");