# Ruby Singleton Implementation Example
class Singleton
  @@instance = nil

  def self.instance
    @@instance ||= new
  end

  private_class_method :new
end