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

    public void Run(IPresenter presenter, IRandomGenerator randomGenerator)
    {
        season.NewSeason();
        query.SetTeams(season.Teams);

        const int NumberOfRaces = 10;
        for (uint raceNumber = 0; raceNumber < NumberOfRaces; raceNumber++)
        {
            season.DisplayRaceInformation(presenter, raceNumber + 1);
            season.UpdateOverallRaceChances(query, randomGenerator);
            season.RaceResult();
            season.AssignPoints(query, pointsSystem);
            season.DisplayRaceResult(presenter, raceNumber + 1);
        }

        season.DisplaySeasonResult(presenter);
    }
}