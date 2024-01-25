using Moq;
using Xunit;

public class SeasonServiceShould
{
    [Fact]
    public void Run()
    {
        // Given
        Mock<IPresenter> presenter = new();
        Mock<ITeam> team = new();
        Mock<IQuery> query = new();
        Mock<ISeason> season = new();
        Mock<IPointsSystem> pointsSystem = new();
        season.Setup(s => s.NewSeason());
        season.Setup(s => s.Teams).Returns(new ITeam[] { team.Object });
        query.Setup(q => q.SetTeams(season.Object.Teams));
        season.Setup(s => s.DisplayRaceInformation(presenter.Object, 1));
        season.Setup(s => s.DisplayRaceInformation(presenter.Object, 10));
        season.Setup(s => s.UpdateOverallRaceChances(query.Object));
        season.Setup(s => s.RaceResult());
        season.Setup(s => s.AssignPoints(query.Object, pointsSystem.Object));
        season.Setup(s => s.DisplayRaceResult(presenter.Object, 1));
        season.Setup(s => s.DisplayRaceResult(presenter.Object, 10));
        season.Setup(s => s.DisplaySeasonResult(presenter.Object));

        ISeasonService seasonService = new SeasonService(season.Object, query.Object, pointsSystem.Object);

        // When
        seasonService.Run(presenter.Object);

        // Then
        season.VerifyAll();
        query.VerifyAll();
        pointsSystem.VerifyAll();
    }
}