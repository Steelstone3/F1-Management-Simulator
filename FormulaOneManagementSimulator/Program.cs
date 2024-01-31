namespace BubblesDivePlanner
{
    class Program
    {
        internal static void Main()
        {
            IRandomGenerator randomGenerator = new RandomGenerator();
            IDriverFactory driverFactory = new DriverFactory(randomGenerator);
            ITeamFactory teamFactory = new TeamFactory(randomGenerator);
            IPresenter presenter = new Presenter();
            ISeason season = new Season(driverFactory, teamFactory);
            IQuery query = new Query(season.Teams);
            IPointsSystem pointsSystem = new PointsSystem();

            ISeasonService seasonService = new SeasonService(season, query, pointsSystem);

            seasonService.Run(presenter, randomGenerator);
        }
    }
}
