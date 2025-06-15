# Ruby Factory Pattern Example

class Product
  def operation
    raise NotImplementedError
  end
end

class ConcreteProductA < Product
  def operation
    'Result of ConcreteProductA'
  end
end

class ConcreteProductB < Product
  def operation
    'Result of ConcreteProductB'
  end
end

class Creator
  def factory_method
    raise NotImplementedError
  end

  def some_operation
    product = factory_method
    product.operation
  end
end

class ConcreteCreatorA < Creator
  def factory_method
    ConcreteProductA.new
  end
end

class ConcreteCreatorB < Creator
  def factory_method
    ConcreteProductB.new
  end
end