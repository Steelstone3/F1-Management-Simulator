public class Team : ITeam
{
    public Team(string name, string driverOneName, string driverTwoName, ITeamRating teamRating, ICarRating carRating, IPoints points)
    {
        Name = name;
        DriverOneName = driverOneName;
        DriverTwoName = driverTwoName;
        TeamRating = teamRating;
        CarRating = carRating;
        Points = points;
    }

    public string Name { get; }
    public string DriverOneName { get; }
    public string DriverTwoName { get; }
    public ITeamRating TeamRating { get; }
    public ICarRating CarRating { get; }
    public IPoints Points { get; }

    public void AddPoints(uint points)
    {
        Points.AddPoints(points);
    }

    public void Display(IPresenter presenter)
    {
        presenter.Display($"| {Name} | {DriverOneName} | {DriverTwoName} | {Points.SeasonPoints} Points |");
    }
}