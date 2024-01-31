public class Driver : IDriver
{
    public Driver(string name, string team, IDriverRating driverRating, IPoints points)
    {
        Name = name;
        Team = team;
        DriverRating = driverRating;
        Points = points;
    }

    public string Name { get; }
    public string Team { get; }
    public IDriverRating DriverRating { get; }
    public IPoints Points { get; }

    public void AddPoints(IQuery query, uint racePoints)
    {
        Points.AddPoints(racePoints);

        ITeam team = query.FindTeam(Team);
        team.AddPoints(racePoints);
    }

    public void UpdateOverallRaceChance(IQuery query)
    {
        ITeam team = query.FindTeam(Team);

        DriverRating.UpdateOverallRaceChance(team);
    }

    public void DisplayRace(IPresenter presenter, uint raceNumber)
    {
        presenter.Display($"| {Name} | {Team} | {Points.RacePoints[(int)raceNumber]} Points |");
    }

    public void DisplaySeason(IPresenter presenter)
    {
        presenter.Display($"| {Name} | {Team} | {Points.SeasonPoints} Points |");
    }
}