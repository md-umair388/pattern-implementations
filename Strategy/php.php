<?php
// PHP Strategy Pattern Example

interface Strategy {
    public function execute($a, $b);
}

class AddStrategy implements Strategy {
    public function execute($a, $b) {
        return $a + $b;
    }
}

class SubtractStrategy implements Strategy {
    public function execute($a, $b) {
        return $a - $b;
    }
}

class Context {
    private $strategy;
    public function __construct(Strategy $strategy) {
        $this->strategy = $strategy;
    }
    public function setStrategy(Strategy $strategy) {
        $this->strategy = $strategy;
    }
    public function executeStrategy($a, $b) {
        return $this->strategy->execute($a, $b);
    }
}

// Usage:
// $context = new Context(new AddStrategy());
// echo $context->executeStrategy(5, 3); // 8
// $context->setStrategy(new SubtractStrategy());
// echo $context->executeStrategy(5, 3); // 2
?>