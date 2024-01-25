public interface IPoints
{
    uint SeasonPoints { get; }
    IList<uint> RacePoints { get; }

    void AddPoints(uint racePoints);
}