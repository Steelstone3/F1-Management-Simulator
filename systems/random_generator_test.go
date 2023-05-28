package systems

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGenerateRandomInRange1(t *testing.T) {
	// Given
	var expected uint = 82
	var seed int64 = 1234
	var minimumRange uint
	var maximumRange uint = 100

	// When
	value := GenerateRandomInRange(seed, minimumRange, maximumRange)

	// Then
	assert.Equal(t, expected, value)
}

func TestGenerateRandomInRange2(t *testing.T) {
	// Given
	var expected uint = 42
	var seed int64 = 4321
	var minimumRange uint
	var maximumRange uint = 100

	// When
	value := GenerateRandomInRange(seed, minimumRange, maximumRange)

	// Then
	assert.Equal(t, expected, value)
}

func TestGenerateRandomInRange3(t *testing.T) {
	// Given
	var expected uint = 94
	var seed int64 = 12344321
	var minimumRange uint
	var maximumRange uint = 100

	// When
	value := GenerateRandomInRange(seed, minimumRange, maximumRange)

	// Then
	assert.Equal(t, expected, value)
}

func TestGetRandomString1(t *testing.T) {
	// Given
	expected := "Pear"
	var seed int64 = 12344321
	strings := []string{"Apple", "Pear", "Banana"}

	// When
	value := GetRandomString(seed, strings)

	// Then
	assert.Equal(t, expected, value)
}

func TestGetRandomString2(t *testing.T) {
	// Given
	expected := "Banana"
	var seed int64 = 1234
	strings := []string{"Apple", "Pear", "Banana"}

	// When
	value := GetRandomString(seed, strings)

	// Then
	assert.Equal(t, expected, value)
}

func TestGetRandomString3(t *testing.T) {
	// Given
	expected := "Apple"
	var seed int64 = 4321
	strings := []string{"Apple", "Pear", "Banana"}

	// When
	value := GetRandomString(seed, strings)

	// Then
	assert.Equal(t, expected, value)
}
