public class SeasonService : ISeasonService
{
    private readonly ISeason season;
    private readonly IQuery query;
    private readonly IPointsSystem pointsSystem;

    public SeasonService(ISeason season, IQuery query, IPointsSystem pointsSystem)
    {
        this.season = season;
        this.query = query;
        this.pointsSystem = pointsSystem;
    }

    public void Run(IPresenter presenter)
    {
        season.NewSeason();
        query.SetTeams(season.Teams);

        // TODO make a NumberOfRaces constant
        for (uint raceNumber = 0; raceNumber < 10; raceNumber++)
        {
            season.DisplayRaceInformation(presenter, raceNumber + 1);
            season.UpdateOverallRaceChances(query);
            season.RaceResult();
            season.AssignPoints(query, pointsSystem);
            season.DisplayRaceResult(presenter, raceNumber + 1);
        }

        season.DisplaySeasonResult(presenter);
    }
}