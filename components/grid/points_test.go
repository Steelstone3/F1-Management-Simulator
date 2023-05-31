package grid

import (
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
	"testing"
)

type TestSuite struct {
	suite.Suite
}

func (pointsTestSuite *TestSuite) TestCalculatePoints() {
	testCases := []struct {
		name           string
		position       Position
		expectedPoints Points
	}{
		{"Case 1: First place", Position{1}, Points{points: 25}},
		{"Case 2: Second place", Position{2}, Points{points: 18}},
		{"Case 3: Third place", Position{3}, Points{points: 15}},
		{"Case 4: Fourth place", Position{4}, Points{points: 12}},
		{"Case 5: Fifth place", Position{5}, Points{points: 10}},
		{"Case 6: Sixth place", Position{6}, Points{points: 8}},
		{"Case 7: Seventh place", Position{7}, Points{points: 6}},
		{"Case 8: Eighth place", Position{8}, Points{points: 4}},
		{"Case 9: Ninth place", Position{9}, Points{points: 2}},
		{"Case 10: Tenth place", Position{10}, Points{points: 1}},
		{"Case 11: Eleventh place", Position{11}, Points{points: 0}},
		{"Case 12: Twentieth place", Position{20}, Points{points: 0}},
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
