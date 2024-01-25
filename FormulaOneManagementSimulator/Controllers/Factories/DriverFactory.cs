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
            new Driver("Piere Gasley", "Alpine", DriverRatingFactory(), PointsFactory()),
            new Driver("Esteban Ocon", "Alpine", DriverRatingFactory(), PointsFactory()),
            new Driver("Alex Albon", "Williams", DriverRatingFactory(), PointsFactory()),
            new Driver("Logan Sargent", "Williams", DriverRatingFactory(), PointsFactory()),
        };
    }

    private IDriverRating DriverRatingFactory()
    {
        var seeds = randomGenerator.GenerateSeeds(5);
        var randomNumbers = randomGenerator.Generate(seeds);

        return new DriverRating(randomNumbers);
    }

    private IPoints PointsFactory() => new Points();
}