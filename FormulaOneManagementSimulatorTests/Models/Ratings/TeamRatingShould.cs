using Xunit;

public class TeamRatingShould
{
    [Theory]
    [InlineData(new uint[5]{50, 51, 52, 53, 54}, 52)]
    [InlineData(new uint[5]{90, 92, 65, 51, 85}, 76)]
    [InlineData(new uint[5]{55, 65, 75, 99, 78}, 74)]
    public void CreateNewTeamRating(uint[] ratings, uint overall)
    {
        // Given
        ITeamRating teamRating = new TeamRating
        (
           ratings
        );

        // Then
        Assert.Equal(ratings[0], teamRating.InSeasonDevelopment);
        Assert.Equal(ratings[1], teamRating.Repairs);
        Assert.Equal(ratings[2], teamRating.RaceSetup);
        Assert.Equal(ratings[3], teamRating.Strategy);
        Assert.Equal(ratings[4], teamRating.Management);
        Assert.Equal(overall, teamRating.Overall);
    }
}

