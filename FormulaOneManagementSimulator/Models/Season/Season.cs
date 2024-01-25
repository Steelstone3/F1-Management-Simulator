public class Season : ISeason
{
    private readonly IDriverFactory driverFactory;
    private readonly ITeamFactory teamFactory;

    public Season(IDriverFactory driverFactory, ITeamFactory teamFactory)
    {
        this.driverFactory = driverFactory;
        this.teamFactory = teamFactory;
        NewSeason();
    }

    public IDriver[] Drivers { get; private set; }
    public ITeam[] Teams { get; private set; }

    public void NewSeason()
    {
        Drivers = driverFactory.Create();
        Teams = teamFactory.Create();
    }

    public void DisplayRaceInformation(IPresenter presenter, uint raceNumber)
    {
        // TODO make a NumberOfRaces constant
        string[] raceNames = new string[10]
        {
            "Spa-Francorchamps",
            "Silverstone",
            "Imola",
            "Monza",
            "Magny-Cours",
            "Hockenheim",
            "Österreichring",
            "Monaco",
            "Zandvoort",
            "Interlagos"
        };

        presenter.Display($"\nRace {raceNumber} {raceNames[raceNumber - 1]}\n");
    }

    public void UpdateOverallRaceChances(IQuery query)
    {
        foreach (IDriver driver in Drivers)
        {
            driver.UpdateOverallRaceChance(query);
        }
    }

    public void RaceResult()
    {
        Drivers = Drivers.OrderByDescending(driver => driver.DriverRating.OverallRaceChance).ToArray();
    }

    public void AssignPoints(IQuery query, IPointsSystem pointsSystem)
    {
        for (uint finishPosition = 0; finishPosition < 10; finishPosition++)
        {
            Drivers[finishPosition].AddPoints(query, pointsSystem.PointsForFinishPosition(finishPosition + 1));
        }
    }

    public void DisplayRaceResult(IPresenter presenter, uint raceNumber)
    {
        foreach (IDriver driver in Drivers)
        {
            driver.Display(presenter, raceNumber - 1);
        }
    }

    public void DisplaySeasonResult(IPresenter presenter)
    {
        presenter.Display("\nSeason Result\n");

        foreach (ITeam team in Teams)
        {
            team.Display(presenter);
        }
    }
}