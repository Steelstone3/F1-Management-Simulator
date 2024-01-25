using Moq;
using Xunit;

public class DriverFactoryShould
{
    [Fact]
    public void CreateTheDrivers()
    {
        // Given
        Mock<IRandomGenerator> randomGenerator = new();
        int[] seeds = new int[5] { 1, 2, 3, 4, 5 };
        uint[] ratings = new uint[5] { 50, 50, 50, 50, 50 };
        randomGenerator.Setup(rg => rg.GenerateSeeds(5)).Returns(seeds);
        randomGenerator.Setup(rg => rg.Generate(seeds)).Returns(ratings);
        IDriverFactory driverFactory = new DriverFactory(randomGenerator.Object);

        // When
        IDriver[] drivers = driverFactory.Create();

        // Then
        randomGenerator.VerifyAll();
        randomGenerator.VerifyNoOtherCalls();
        Assert.NotEmpty(drivers);
        Assert.NotSame(drivers[0].Points, drivers[1].Points);
        Assert.NotSame(drivers[0].DriverRating, drivers[1].DriverRating);
    }
}