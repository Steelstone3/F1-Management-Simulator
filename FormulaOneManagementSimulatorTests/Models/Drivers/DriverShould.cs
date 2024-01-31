using Moq;
using Xunit;

public class DriverShould
{
    private readonly Mock<IPoints> points = new();
    private readonly Mock<IDriverRating> driverRating = new();
    private IDriver driver;

    public DriverShould()
    {
        driver = new Driver("Lewis Hamilton", "Mercedes", driverRating.Object, points.Object);
    }

    [Theory]
    [InlineData("Lewis Hamilton", "Mercedes")]
    [InlineData("George Russell", "Mercedes")]
    [InlineData("Piere Gasley", "Alpine")]
    public void CreateNewDriver(string name, string team)
    {
        // Given
        IDriver driver = new Driver(name, team, driverRating.Object, points.Object);

        // Then
        Assert.Equal(name, driver.Name);
        Assert.Equal(team, driver.Team);
        Assert.Equal(driverRating.Object, driver.DriverRating);
        Assert.Equal(points.Object, driver.Points);
    }

    [Fact]
    public void AddPoints()
    {
        // Given
        uint racePoints = 10;
        points.Setup(p => p.AddPoints(racePoints));

        Mock<ITeam> team = new();
        team.Setup(t => t.AddPoints(racePoints));

        driver = new Driver("Lewis Hamilton", "Mercedes", driverRating.Object, points.Object);

        Mock<IQuery> query = new();
        query.Setup(q => q.FindTeam(driver.Team)).Returns(team.Object);

        // When
        driver.AddPoints(query.Object, racePoints);

        // Then
        query.VerifyAll();
        query.VerifyNoOtherCalls();
        team.VerifyAll();
        team.VerifyNoOtherCalls();
        points.VerifyAll();
        points.VerifyNoOtherCalls();
    }

    [Fact]
    public void CalculateOverallRaceChance()
    {
        // Given
        Mock<IRandomGenerator> randomGenerator = new();

        Mock<ITeam> team = new();
        driverRating.Setup(dr => dr.UpdateOverallRaceChance(team.Object, randomGenerator.Object));

        Mock<IQuery> query = new();
        query.Setup(q => q.FindTeam(driver.Team)).Returns(team.Object);

        // When
        driver.UpdateOverallRaceChance(query.Object, randomGenerator.Object);

        // Then
        driverRating.VerifyAll();
        driverRating.VerifyNoOtherCalls();
        query.VerifyAll();
        query.VerifyNoOtherCalls();
    }

    [Fact]
    public void DisplayRaceResult()
    {
        // Given
        uint raceNumber = 0;
        List<uint> racePoints = new List<uint>() { 50 };
        Mock<IPresenter> presenter = new();
        presenter.Setup(p => p.Display("| Lewis Hamilton | Mercedes | 50 Points |"));

        Mock<IPoints> points = new();
        points.Setup(p => p.RacePoints).Returns(racePoints);

        driver = new Driver("Lewis Hamilton", "Mercedes", driverRating.Object, points.Object);

        // When
        driver.DisplayRace(presenter.Object, raceNumber);

        // Then
        presenter.VerifyAll();
    }

    [Fact]
    public void DisplaySeasonResult()
    {
        // Given

        uint seasonPoints = 50;
        Mock<IPresenter> presenter = new();
        presenter.Setup(p => p.Display("| Lewis Hamilton | Mercedes | 50 Points |"));

        Mock<IPoints> points = new();
        points.Setup(p => p.SeasonPoints).Returns(seasonPoints);

        driver = new Driver("Lewis Hamilton", "Mercedes", driverRating.Object, points.Object);

        // When
        driver.DisplaySeason(presenter.Object);

        // Then
        presenter.VerifyAll();
    }
}