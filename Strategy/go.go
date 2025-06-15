// Go Strategy Pattern Example

package strategy

type Strategy interface {
    Execute(a, b int) int
}

type AddStrategy struct{}
func (AddStrategy) Execute(a, b int) int { return a + b }

type SubtractStrategy struct{}
func (SubtractStrategy) Execute(a, b int) int { return a - b }

type Context struct {
    strategy Strategy
}

func NewContext(s Strategy) *Context {
    return &Context{strategy: s}
}

func (c *Context) SetStrategy(s Strategy) {
    c.strategy = s
}

func (c *Context) ExecuteStrategy(a, b int) int {
    return c.strategy.Execute(a, b)
}

// Usage:
// context := NewContext(AddStrategy{})
// fmt.Println(context.ExecuteStrategy(5, 3)) // 8
// context.SetStrategy(SubtractStrategy{})
// fmt.Println(context.ExecuteStrategy(5, 3)) // 2