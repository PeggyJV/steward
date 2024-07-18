package builder

// Tests for the CellarCallBuilder

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Test the CellarCallDataBuilder constructor
func TestNewCellarCallData(t *testing.T) {
	// Create a new CellarCallDataBuilder
	builder := NewCellarCallDataBuilder()

	// Check the builder
	assert.Equal(t, 0, len(builder.functionCalls))

	// Can't build an empty builder
	result, error := builder.Build()
	assert.Nil(t, result)
	assert.Error(t, error)
}

// Test the CallDataBuilder
