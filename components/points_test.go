package components

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
		expectedPoints Points
	}{
		{"Case 1: First place", Points{position: 1, points: 25}},
		{"Case 2: Second place", Points{position: 2, points: 18}},
		{"Case 3: Third place", Points{position: 3, points: 15}},
		{"Case 4: Fourth place", Points{position: 4, points: 12}},
		{"Case 5: Fifth place", Points{position: 5, points: 10}},
		{"Case 6: Sixth place", Points{position: 6, points: 8}},
		{"Case 7: Seventh place", Points{position: 7, points: 6}},
		{"Case 8: Eighth place", Points{position: 8, points: 4}},
		{"Case 9: Ninth place", Points{position: 9, points: 2}},
		{"Case 10: Tenth place", Points{position: 10, points: 1}},
		{"Case 11: Eleventh place", Points{position: 11, points: 0}},
		{"Case 12: Twentieth place", Points{position: 20, points: 0}},
	}

	for _, tc := range testCases {
		pointsTestSuite.Run(tc.name, func() {
			points := calculatePoints(tc.expectedPoints.position)
			assert.Equal(pointsTestSuite.T(), tc.expectedPoints, points)
		})
	}
}

func TestRunTestSuite(t *testing.T) {
	suite.Run(t, new(TestSuite))
}
