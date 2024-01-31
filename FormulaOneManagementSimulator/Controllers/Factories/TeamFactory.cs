public class TeamFactory : ITeamFactory
{
    private readonly IRandomGenerator randomGenerator;

    public TeamFactory(IRandomGenerator randomGenerator)
    {
        this.randomGenerator = randomGenerator;
    }

    public ITeam[] Create()
    {
        return new ITeam[]
        {
            new Team("Mercedes", "Lewis Hamilton", "George Russell", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Red Bull", "Max Verstappen", "Sergio Perez", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Ferrari", "Charles Lecerc", "Carlos Sainz", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("McLaren", "Lando Norris", "Oscar Piastri", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Aston Martin", "Fernando Alonso", "Lance Stroll", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Alpine", "Esteban Ocon", "Piere Gasley", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Williams", "Alex Albon", "Logan Sargent", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Racing Bulls", "Daniel Ricciardo", "Yuki Tsunoda", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Sauber", "Valtteri Bottas", "Guanyu Zhou", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),
            new Team("Haas", "Nico Hulkenberg", "Kevin Magnussen", TeamRatingFactory(), CarRatingFactory(), PointsFactory()),         
        };
    }

    private ITeamRating TeamRatingFactory()
    {
        int[] seeds = randomGenerator.GenerateSeeds(5);
        uint[] ratings = randomGenerator.Generate(seeds);

        return new TeamRating(ratings);
    }

    private ICarRating CarRatingFactory()
    {
        int[] seeds = randomGenerator.GenerateSeeds(4);
        uint[] ratings = randomGenerator.Generate(seeds);

        return new CarRating(ratings);
    }

    private IPoints PointsFactory() => new Points();
}