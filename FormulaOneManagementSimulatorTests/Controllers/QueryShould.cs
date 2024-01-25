using Moq;
using Xunit;

public class QueryShould
{
    [Fact]
    public void FindTeam()
    {
        // Given
        Mock<ITeam> team = new();
        team.Setup(t => t.Name).Returns("Mercedes");
        ITeam[] teams = new ITeam[] { team.Object };

        IQuery query = new Query(teams);

        // When
        ITeam actualTeam = query.FindTeam("Mercedes");

        // Then
        Assert.Equal(team.Object, actualTeam);
    }

    [Fact]
    public void CanNotFindTeam()
    {
        // Given
        Mock<ITeam> team = new();
        ITeam[] teams = new ITeam[] { team.Object };

        IQuery query = new Query(teams);

        // Then
        Assert.Throws<ArgumentOutOfRangeException>(() => query.FindTeam("Mercedes"));
    }
}