using Xunit;

public class PointsSystemShould
{
    [Theory]
    [InlineData(0, 0)]
    [InlineData(11, 0)]
    [InlineData(1, 25)]
    [InlineData(2, 18)]
    [InlineData(3, 15)]
    [InlineData(4, 12)]
    [InlineData(5, 10)]
    [InlineData(6, 8)]
    [InlineData(7, 6)]
    [InlineData(8, 4)]
    [InlineData(9, 2)]
    [InlineData(10, 1)]
    public void CalculatePoints(uint finishPosition, uint expectedPoints)
    {
        // Given
        IPointsSystem pointsSystem = new PointsSystem();

        // When
        uint points = pointsSystem.PointsForFinishPosition(finishPosition);

        // Then
        Assert.Equal(expectedPoints, points);
    }
}