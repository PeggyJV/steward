package builder

// Tests for the CellarCallBuilder

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Test the CallDataBuilder constructor
func TestNewCallData(t *testing.T) {
	// Create a new CellarCallBuilder
	builder := NewCellarCallDataBuilder()

	// Check the builder
	assert.Equal(t, 0, len(builder.functionCalls))

	// Can't build an empty builder
	result, error := builder.Build()
	assert.Nil(t, result)
	assert.Error(t, error)
}

// Test the CallDataBuilder
