using Moq;
using Xunit;

public class TeamFactoryShould
{
    [Fact]
    public void CreateTheTeams()
    {
        // Given
        Mock<IRandomGenerator> randomGenerator = new();
        int[] teamRatingSeeds = new int[5] { 1, 2, 3, 4, 5 };
        int[] carRatingSeeds = new int[4] { 10, 20, 30, 40 };
        uint[] teamRatings = new uint[5] { 50, 50, 50, 50, 50 };
        uint[] carRatings = new uint[4] { 60, 60, 60, 60 };
        randomGenerator.Setup(rg => rg.GenerateSeeds(5)).Returns(teamRatingSeeds);
        randomGenerator.Setup(rg => rg.Generate(teamRatingSeeds)).Returns(teamRatings);
        randomGenerator.Setup(rg => rg.GenerateSeeds(4)).Returns(carRatingSeeds);
        randomGenerator.Setup(rg => rg.Generate(carRatingSeeds)).Returns(carRatings);
        ITeamFactory teamFactory = new TeamFactory(randomGenerator.Object);

        // When
        ITeam[] teams = teamFactory.Create();

        // Then
        randomGenerator.VerifyAll();
        randomGenerator.VerifyNoOtherCalls();
        Assert.NotEmpty(teams);
        Assert.NotSame(teams[0].TeamRating, teams[1].TeamRating);
        Assert.NotSame(teams[0].CarRating, teams[1].CarRating);
        Assert.NotSame(teams[0].Points, teams[1].Points);
    }
}