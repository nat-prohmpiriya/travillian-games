package validator

import (
	"sync"

	"github.com/go-playground/validator/v10"
)

var (
	validate *validator.Validate
	once     sync.Once
)

func Get() *validator.Validate {
	once.Do(func() {
		validate = validator.New()
	})
	return validate
}

func ValidateStruct(s interface{}) error {
	return Get().Struct(s)
}
