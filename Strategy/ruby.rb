# Ruby Strategy Pattern Example

class Strategy
  def execute(a, b)
    raise NotImplementedError
  end
end

class AddStrategy < Strategy
  def execute(a, b)
    a + b
  end
end

class SubtractStrategy < Strategy
  def execute(a, b)
    a - b
  end
end

class Context
  def initialize(strategy)
    @strategy = strategy
  end

  def set_strategy(strategy)
    @strategy = strategy
  end

  def execute_strategy(a, b)
    @strategy.execute(a, b)
  end
end

# Usage:
# context = Context.new(AddStrategy.new)
# puts context.execute_strategy(5, 3) # 8
# context.set_strategy(SubtractStrategy.new)
# puts context.execute_strategy(5, 3) # 2