class Calculator
  def initialize
    @multiplier = 2
  end

  def add(a, b)
    return a + b
  end

  def self.multiply(a, b)
    a * b
  end

  def apply_operation(&block)
    block.call(@multiplier)
  end
end

# Lambda example
square = ->(x) { x * x }

# Proc example
double = Proc.new { |x| x * 2 }

# Block iteration
[1, 2, 3].each do |n|
  puts n * 2
end

calc = Calculator.new
result = calc.add(5, 3)
puts result

class_result = Calculator.multiply(4, 5)
puts class_result

lambda_result = square.call(5)
puts lambda_result
