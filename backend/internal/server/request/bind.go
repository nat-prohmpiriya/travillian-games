package request

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/travillian/tusk-horn/internal/pkg/validator"
)

func Bind(r *http.Request, v interface{}) error {
	if err := json.NewDecoder(r.Body).Decode(v); err != nil {
		return fmt.Errorf("invalid request body: %w", err)
	}
	defer r.Body.Close()

	if err := validator.ValidateStruct(v); err != nil {
		return fmt.Errorf("validation failed: %w", err)
	}

	return nil
}
