public class DriverRating : IDriverRating
{
    public DriverRating(uint[] ratings)
    {
        Awareness = ratings[0];
        Consistency = ratings[1];
        Experience = ratings[2];
        RaceCraft = ratings[3];
        Pace = ratings[4];
        CalculateOverall();
    }

    public uint Awareness { get; }
    public uint Consistency { get; }
    public uint Experience { get; }
    public uint RaceCraft { get; }
    public uint Pace { get; }
    public uint Overall { get; private set; }
    public uint OverallRaceChance {get; private set;}

    public void UpdateOverallRaceChance(ITeam team)
    {
        OverallRaceChance = (Overall + team.TeamRating.Overall + team.CarRating.Overall) / 3;
    }

    private void CalculateOverall()
    {
        Overall = (Awareness + Consistency + Experience + RaceCraft + Pace) / 5;
    }
}