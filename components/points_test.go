package components

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCalculatePoints(t *testing.T) {
	// Given
	var position = Position {
		position: 1,
	}
	var expectedPoints = Points{
		points: 25,
	}

	// When
	var points = calculatePoints(position);

	// Then
	assert.Equal(t, expectedPoints, points)
}
