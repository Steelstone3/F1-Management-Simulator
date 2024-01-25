public class TeamRating : ITeamRating
{
    public TeamRating(uint[] ratings)
    {
        InSeasonDevelopment = ratings[0];
        Repairs = ratings[1];
        RaceSetup = ratings[2];
        Strategy = ratings[3];
        Management = ratings[4];
        CalculateOverall();
    }

    public uint InSeasonDevelopment { get; }
    public uint Repairs { get; }
    public uint RaceSetup { get; }
    public uint Strategy { get; }
    public uint Management { get; }
    public uint Overall { get; private set; }

    private void CalculateOverall()
    {
        Overall = (InSeasonDevelopment + Repairs + RaceSetup + Strategy + Management) / 5;
    }
}