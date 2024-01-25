using Xunit;

public class CarRatingShould
{
    [Theory]
    [InlineData(new uint[4] { 50, 51, 52, 53 }, 51)]
    [InlineData(new uint[4] { 90, 92, 65, 51 }, 74)]
    [InlineData(new uint[4] { 55, 65, 75, 99 }, 73)]
    public void CreateNewCarRating(
        uint[] ratings,
        uint overall
    )
    {
        // Given
        ICarRating carRating = new CarRating
        (
            ratings
        );

        // Then
        Assert.Equal(ratings[0], carRating.Aero);
        Assert.Equal(ratings[1], carRating.Engine);
        Assert.Equal(ratings[2], carRating.Reliability);
        Assert.Equal(ratings[3], carRating.TireManagement);
        Assert.Equal(overall, carRating.Overall);
    }
}