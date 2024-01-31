using Moq;
using Xunit;

public class SeasonShould
{
    [Fact]
    public void CreateSeason()
    {
        // Given
        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create());

        Mock<ITeamFactory> teamFactory = new();
        teamFactory.Setup(tf => tf.Create());

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // Then
        driverFactory.VerifyAll();
        teamFactory.VerifyAll();
    }

    [Fact]
    public void DisplayRaceInformation()
    {
        // Given
        Mock<IDriverFactory> driverFactory = new();
        Mock<ITeamFactory> teamFactory = new();

        Mock<IPresenter> presenter = new();
        presenter.Setup(p => p.Display("\nRace 1 Spa-Francorchamps\n"));

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.DisplayRaceInformation(presenter.Object, 1);

        // Then
        presenter.VerifyAll();
        presenter.VerifyNoOtherCalls();
    }

    [Fact]
    public void NewSeason()
    {
        // Given
        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create());

        Mock<ITeamFactory> teamFactory = new();
        teamFactory.Setup(tf => tf.Create());

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.NewSeason();

        // Then
        driverFactory.VerifyAll();
        teamFactory.VerifyAll();
    }

    [Fact]
    public void UpdateOverallRaceChances()
    {
        // Given
        Mock<IRandomGenerator> randomGenerator = new();
        Mock<IQuery> query = new();
        Mock<IDriver> driver = new();
        driver.Setup(d => d.UpdateOverallRaceChance(query.Object, randomGenerator.Object));

        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create()).Returns(new IDriver[] { driver.Object, driver.Object });

        Mock<ITeamFactory> teamFactory = new();

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.UpdateOverallRaceChances(query.Object, randomGenerator.Object);

        // Then
        driver.VerifyAll();
        driver.VerifyNoOtherCalls();
        driverFactory.VerifyAll();
        driverFactory.VerifyNoOtherCalls();
    }

    [Fact]
    public void RaceResult()
    {
        // Given
        Mock<IDriverRating> driverRating1 = new();
        driverRating1.Setup(dr => dr.OverallRaceChance).Returns(99);
        Mock<IDriver> driver1 = new();
        driver1.Setup(d => d.DriverRating).Returns(driverRating1.Object);

        Mock<IDriverRating> driverRating2 = new();
        driverRating2.Setup(dr => dr.OverallRaceChance).Returns(80);
        Mock<IDriver> driver2 = new();
        driver2.Setup(d => d.DriverRating).Returns(driverRating2.Object);

        Mock<IDriverRating> driverRating3 = new();
        driverRating3.Setup(dr => dr.OverallRaceChance).Returns(70);
        Mock<IDriver> driver3 = new();
        driver3.Setup(d => d.DriverRating).Returns(driverRating3.Object);

        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create()).Returns(new IDriver[] { driver1.Object, driver2.Object, driver3.Object });

        Mock<ITeamFactory> teamFactory = new();

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.RaceResult();

        // Then
        driverFactory.VerifyAll();
        driverFactory.VerifyNoOtherCalls();
        driver1.VerifyAll();
        driver1.VerifyNoOtherCalls();
        driverRating1.VerifyAll();
        driverRating1.VerifyNoOtherCalls();
        driver2.VerifyAll();
        driver2.VerifyNoOtherCalls();
        driverRating2.VerifyAll();
        driverRating2.VerifyNoOtherCalls();
        driver3.VerifyAll();
        driver3.VerifyNoOtherCalls();
        driverRating3.VerifyAll();
        driverRating3.VerifyNoOtherCalls();
    }

    [Fact]
    public void AssignPoints()
    {
        // Given
        Mock<IQuery> query = new();

        Mock<IPointsSystem> pointsSystem = new();
        pointsSystem.Setup(ps => ps.PointsForFinishPosition(1)).Returns(25);

        Mock<IDriver> driver = new();
        driver.Setup(d => d.AddPoints(query.Object, 25));

        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create()).Returns(new IDriver[20] { driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object, driver.Object });

        Mock<ITeamFactory> teamFactory = new();

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.AssignPoints(query.Object, pointsSystem.Object);

        // Then
        driverFactory.VerifyAll();
        driverFactory.VerifyNoOtherCalls();
        driver.VerifyAll();
        pointsSystem.VerifyAll();
    }

    [Fact]
    public void DisplayRaceResult()
    {
        // Given
        Mock<IPresenter> presenter = new();

        Mock<IDriver> driver = new();
        driver.Setup(d => d.DisplayRace(presenter.Object, 0));

        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create()).Returns(new IDriver[] { driver.Object });

        Mock<ITeamFactory> teamFactory = new();

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.DisplayRaceResult(presenter.Object, 1);

        // Then
        driverFactory.VerifyAll();
        driverFactory.VerifyNoOtherCalls();
        driver.VerifyAll();
    }

    [Fact]
    public void DisplaySeasonResult()
    {
        // Given
        Mock<IPresenter> presenter = new();
        presenter.Setup(p => p.Display("\nSeason Result\n"));
        presenter.Setup(p => p.Display("Constructor's Championship\n"));
        presenter.Setup(p => p.Display("\nDriver's Championship\n"));

        Mock<IPoints> points = new();

        Mock<ITeam> team = new();
        team.Setup(t => t.Points).Returns(points.Object);
        team.Setup(t => t.Display(presenter.Object));

        Mock<ITeamFactory> teamFactory = new();
        teamFactory.Setup(tf => tf.Create()).Returns(new ITeam[] { team.Object });

        Mock<IDriver> driver = new();
        driver.Setup(d => d.Points).Returns(points.Object);
        driver.Setup(d => d.DisplaySeason(presenter.Object));

        Mock<IDriverFactory> driverFactory = new();
        driverFactory.Setup(df => df.Create()).Returns(new IDriver[] { driver.Object });

        ISeason season = new Season(driverFactory.Object, teamFactory.Object);

        // When
        season.DisplaySeasonResult(presenter.Object);

        // Then
        teamFactory.VerifyAll();
        teamFactory.VerifyNoOtherCalls();
        driverFactory.VerifyAll();
        driverFactory.VerifyNoOtherCalls();
        presenter.VerifyAll();
        presenter.VerifyNoOtherCalls();
        team.VerifyAll();
        driver.VerifyAll();
    }
}