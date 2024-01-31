
public interface IDriverRating
{
    uint Awareness { get; }
    uint Consistency { get; }
    uint Experience { get; }
    uint RaceCraft { get; }
    uint Pace { get; }
    uint Overall { get; }
    uint OverallRaceChance { get; }

    void UpdateOverallRaceChance(ITeam team, IRandomGenerator randomGenerator);
}