public interface ISeason
{
    IDriver[] Drivers { get; }
    ITeam[] Teams { get; }
    void NewSeason();
    void DisplayRaceInformation(IPresenter presenter, uint raceNumber);
    void UpdateOverallRaceChances(IQuery query, IRandomGenerator randomGenerator);
    void RaceResult();
    void DisplayRaceResult(IPresenter presenter, uint raceNumber);
    void AssignPoints(IQuery query, IPointsSystem pointsSystem);
    void DisplaySeasonResult(IPresenter presenter);
}