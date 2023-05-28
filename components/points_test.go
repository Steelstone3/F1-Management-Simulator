package components

import (
	"testing"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
)

type TestSuite struct {
	suite.Suite
}

func (pointsTestSuite *TestSuite) TestCalculatePoints() {
	testCases := []struct {
		name     string
		position Position
		expectedPoints Points
	}{
		{"Case 1: First place", Position{position: 1}, Points{points: 25}},
		{"Case 2: Second place", Position{position: 2}, Points{points: 18}},
		{"Case 3: Third place", Position{position: 3}, Points{points: 15}},
		{"Case 4: Fourth place", Position{position: 4}, Points{points: 12}},
		{"Case 5: Fifth place", Position{position: 5}, Points{points: 10}},
		{"Case 6: Sixth place", Position{position: 6}, Points{points: 8}},
		{"Case 7: Seventh place", Position{position: 7}, Points{points: 6}},
		{"Case 8: Eighth place", Position{position: 8}, Points{points: 4}},
		{"Case 9: Ninth place", Position{position: 9}, Points{points: 2}},
		{"Case 10: Tenth place", Position{position: 10}, Points{points: 1}},
	}

	for _, tc := range testCases {
		pointsTestSuite.Run(tc.name, func() {
			points := calculatePoints(tc.position)
			assert.Equal(pointsTestSuite.T(), tc.expectedPoints, points)
		})
	}
}

func TestRunTestSuite(t *testing.T) {
	suite.Run(t, new(TestSuite))
}