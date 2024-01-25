using Xunit;

public class PointsShould
{
    [Fact]
    public void AddPoints()
    {
        // Given
        uint expectedPoints = 25;
        IPoints points = new Points();

        // When
        points.AddPoints(expectedPoints);

        // Then
        Assert.Equal(expectedPoints, points.SeasonPoints);
        Assert.NotEmpty(points.RacePoints);
        Assert.Contains(expectedPoints, points.RacePoints);
    }

     [Fact]
    public void AddMultiplePoints()
    {
        // Given
        uint expectedPoints = 43;
        uint racePoints1 = 25;
        uint racePoints2 = 18;
        IPoints points = new Points();

        // When
        points.AddPoints(racePoints1);
        points.AddPoints(racePoints2);

        // Then
        Assert.Equal(expectedPoints, points.SeasonPoints);
        Assert.NotEmpty(points.RacePoints);
        Assert.Contains(racePoints1, points.RacePoints);
        Assert.Contains(racePoints2, points.RacePoints);
    }
}