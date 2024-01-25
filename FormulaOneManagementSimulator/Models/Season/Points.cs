public class Points : IPoints
{
    public IList<uint> RacePoints { get; private set; } = new List<uint>();
    public uint SeasonPoints { get; private set; }

    public void AddPoints(uint racePoints)
    {
        SeasonPoints += racePoints;
        RacePoints.Add(racePoints);
    }
}