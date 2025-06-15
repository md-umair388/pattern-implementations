# Ruby Observer Pattern Example

class Observer
  def update(message)
    # to be overridden
  end
end

class ConcreteObserver < Observer
  def update(message)
    puts "Observer received: #{message}"
  end
end

class Subject
  def initialize
    @observers = []
  end

  def attach(observer)
    @observers << observer
  end

  def notify(message)
    @observers.each { |observer| observer.update(message) }
  end
end

# Usage:
# subject = Subject.new
# observer = ConcreteObserver.new
# subject.attach(observer)
# subject.notify("Hello Observers!")