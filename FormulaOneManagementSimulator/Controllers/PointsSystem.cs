public class PointsSystem : IPointsSystem
{
    private readonly uint[] pointsAllocation = new uint[10] { 25, 18, 15, 12, 10, 8, 6, 4, 2, 1 };

    public uint PointsForFinishPosition(uint finishPosition)
    {
        if (finishPosition < 1 || finishPosition > 10)
        {
            return 0;
        }

        return pointsAllocation[finishPosition - 1];
    }
}