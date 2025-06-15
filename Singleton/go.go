// Go Singleton Implementation Example
package singleton

var instance *Singleton

type Singleton struct{}

func GetInstance() *Singleton {
    if instance == nil {
        instance = &Singleton{}
    }
    return instance
}