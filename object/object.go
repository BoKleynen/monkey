package object

import "fmt"

type Type string

const (
	INTEGER_OBJ      = "INTEGER"
	BOOLEAN_OBJ      = "BOOLEAN"
	NULL_OBJ         = "NULL"
	RETURN_VALUE_OBJ = "RETURN_VALUE"
	ERROR_OBJ        = "ERROR"
)

type Object interface {
	Type() Type
	Inspect() string
}

type Integer struct {
	Value int64
}

func (i *Integer) Type() Type      { return INTEGER_OBJ }
func (i *Integer) Inspect() string { return fmt.Sprintf("%d", i.Value) }

type Boolean struct {
	Value bool
}

func (b *Boolean) Type() Type      { return BOOLEAN_OBJ }
func (b *Boolean) Inspect() string { return fmt.Sprintf("%t", b.Value) }

type Null struct{}

func (n *Null) Type() Type      { return NULL_OBJ }
func (n *Null) Inspect() string { return "null" }

type ReturnValue struct {
	Value Object
}

func (rv *ReturnValue) Type() Type      { return RETURN_VALUE_OBJ }
func (rv *ReturnValue) Inspect() string { return rv.Value.Inspect() }

type Error struct {
	Message string
}

func (e *Error) Type() Type      { return ERROR_OBJ }
func (e *Error) Inspect() string { return "Error: " + e.Message }

type Environment struct {
	store map[string]Object
}

func NewEnviroment() *Environment {
	s := make(map[string]Object)
	return &Environment{store: s}
}

func (e *Environment) Get(name string) (Object, bool) {
	obj, ok := e.store[name]
	return obj, ok
}

func (e *Environment) Set(name string, val Object) Object {
	e.store[name] = val
	return val
}