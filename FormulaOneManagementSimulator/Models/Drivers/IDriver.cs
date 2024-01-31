public interface IDriver
{
    string Name { get; }
    string Team { get; }
    IDriverRating DriverRating { get; }
    IPoints Points { get; }
    void AddPoints(IQuery query, uint racePoints);
    void UpdateOverallRaceChance(IQuery query, IRandomGenerator randomGenerator);
    void DisplayRace(IPresenter presenter, uint raceNumber);
    void DisplaySeason(IPresenter presenter);
}