public class DriverFactory : IDriverFactory
{
    private readonly IRandomGenerator randomGenerator;

    public DriverFactory(IRandomGenerator randomGenerator)
    {
        this.randomGenerator = randomGenerator;
    }

    public IDriver[] Create()
    {
        return new IDriver[]
        {
            new Driver("Lewis Hamilton", "Mercedes", DriverRatingFactory(), PointsFactory()),
            new Driver("George Russell", "Mercedes", DriverRatingFactory(), PointsFactory()),
            new Driver("Max Verstappen", "Red Bull", DriverRatingFactory(), PointsFactory()),
            new Driver("Sergio Perez", "Red Bull", DriverRatingFactory(), PointsFactory()),
            new Driver("Charles Leclerc", "Ferrari", DriverRatingFactory(), PointsFactory()),
            new Driver("Carlos Sainz", "Ferrari", DriverRatingFactory(), PointsFactory()),
            new Driver("Lando Norris", "McLaren", DriverRatingFactory(), PointsFactory()),
            new Driver("Oscar Piastri", "McLaren", DriverRatingFactory(), PointsFactory()),
            new Driver("Fernando Alonso", "Aston Martin", DriverRatingFactory(), PointsFactory()),
            new Driver("Lance Stroll", "Aston Martin", DriverRatingFactory(), PointsFactory()),
            new Driver("Piere Gasley", "Alpine", DriverRatingFactory(), PointsFactory()),
            new Driver("Esteban Ocon", "Alpine", DriverRatingFactory(), PointsFactory()),
            new Driver("Alex Albon", "Williams", DriverRatingFactory(), PointsFactory()),
            new Driver("Logan Sargent", "Williams", DriverRatingFactory(), PointsFactory()),
            new Driver("Daniel Ricciardo", "Racing Bulls", DriverRatingFactory(), PointsFactory()),
            new Driver("Yuki Tsunoda", "Racing Bulls", DriverRatingFactory(), PointsFactory()),
            new Driver("Valtteri Bottas", "Sauber", DriverRatingFactory(), PointsFactory()),
            new Driver("Guanyu Zhou", "Sauber", DriverRatingFactory(), PointsFactory()),
            new Driver("Nico Hulkenberg", "Haas", DriverRatingFactory(), PointsFactory()),
            new Driver("Kevin Magnussen", "Haas", DriverRatingFactory(), PointsFactory()),
        };
    }

    private IDriverRating DriverRatingFactory()
    {
        int[] seeds = randomGenerator.GenerateSeeds(5);
        uint[] randomNumbers = randomGenerator.Generate(seeds);

        return new DriverRating(randomNumbers);
    }

    private IPoints PointsFactory() => new Points();
}