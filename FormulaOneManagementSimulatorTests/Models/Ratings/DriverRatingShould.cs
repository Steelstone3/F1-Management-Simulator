using Moq;
using Xunit;

public class DriverRatingShould
{
    private readonly IDriverRating driverRating;

    public DriverRatingShould()
    {
        uint[] ratings = { 99, 99, 99, 99, 99 };
        driverRating = new DriverRating(ratings);
    }

    [Theory]
    [InlineData(new uint[5] { 50, 51, 52, 53, 54 }, 52)]
    [InlineData(new uint[5] { 90, 92, 65, 51, 85 }, 76)]
    [InlineData(new uint[5] { 55, 65, 75, 99, 78 }, 74)]
    public void CreateNewDriverRating(uint[] ratings, uint overall)
    {
        // Given
        uint overallRaceChance = 0;
        IDriverRating driverRating = new DriverRating
        (
           ratings
        );

        // Then
        Assert.Equal(ratings[0], driverRating.Awareness);
        Assert.Equal(ratings[1], driverRating.Consistency);
        Assert.Equal(ratings[2], driverRating.Experience);
        Assert.Equal(ratings[3], driverRating.RaceCraft);
        Assert.Equal(ratings[4], driverRating.Pace);
        Assert.Equal(overall, driverRating.Overall);
        Assert.Equal(overallRaceChance, driverRating.OverallRaceChance);
    }

    [Theory]
    [InlineData(79, 89, 79)]
    [InlineData(65, 50, 66)]
    [InlineData(55, 65, 67)]
    public void UpdateOverallRaceChance(uint teamRatingOverall, uint carRatingOverall, uint expectedOverallRaceChance)
    {
        // Given
        int[] seeds = new int[] {12};
        Mock<IRandomGenerator> randomGenerator = new();
        randomGenerator.Setup(rg => rg.GenerateSeeds(1)).Returns(seeds);
        randomGenerator.Setup(rg => rg.Generate(seeds[0])).Returns(50);

        Mock<ITeamRating> teamRating = new();
        teamRating.Setup(dr => dr.Overall).Returns(teamRatingOverall);
        Mock<ICarRating> carRating = new();
        carRating.Setup(dr => dr.Overall).Returns(carRatingOverall);
        Mock<ITeam> team = new();
        team.Setup(t => t.TeamRating).Returns(teamRating.Object);
        team.Setup(t => t.CarRating).Returns(carRating.Object);

        // When
        driverRating.UpdateOverallRaceChance(team.Object, randomGenerator.Object);

        // Then
        team.VerifyAll();
        Assert.Equal(expectedOverallRaceChance, driverRating.OverallRaceChance);
    }
}