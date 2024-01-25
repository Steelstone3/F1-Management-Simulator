using Moq;
using Xunit;

public class TeamShould
{
    private readonly Mock<IPoints> points = new();
    private readonly Mock<ITeamRating> teamRating = new();
    private readonly Mock<ICarRating> carRating = new();
    private ITeam team;

    [Theory]
    [InlineData("Mercedes", "Lewis Hamilton", "George Russell")]
    [InlineData("Alpine", "Esterban Ocon", "Piere Gasly")]
    public void CreateNewDriver(string name, string driverOneName, string driverTwoName)
    {
        // Given
        team = new Team(name, driverOneName, driverTwoName, teamRating.Object, carRating.Object, points.Object);

        // Then
        Assert.Equal(name, team.Name);
        Assert.Equal(driverOneName, team.DriverOneName);
        Assert.Equal(driverTwoName, team.DriverTwoName);
        Assert.Equal(teamRating.Object, team.TeamRating);
        Assert.Equal(carRating.Object, team.CarRating);
        Assert.Equal(points.Object, team.Points);
    }

    [Fact]
    public void AddPoints()
    {
        // Given
        uint racePoints = 10;
        points.Setup(p => p.AddPoints(racePoints));
        team = new Team(null, null, null, null, null, points.Object);

        // When
        team.AddPoints(racePoints);

        // // Then
        points.VerifyAll();
        points.VerifyNoOtherCalls();
    }

    [Fact]
    public void Display()
    {
        // Given
        points.Setup(p => p.SeasonPoints).Returns(50);

        Mock<IPresenter> presenter = new();
        presenter.Setup(p => p.Display("| Mercedes | Lewis Hamilton | George Russell | 50 Points |"));

        team = new Team("Mercedes", "Lewis Hamilton", "George Russell", teamRating.Object, carRating.Object, points.Object);

        // When
        team.Display(presenter.Object);

        // Then
        presenter.VerifyAll();
        presenter.VerifyNoOtherCalls();
        points.VerifyAll();
        points.VerifyNoOtherCalls();
    }
}