public class CarRating : ICarRating
{
    public CarRating
    (
       uint[] ratings
    )
    {
        Aero = ratings[0];
        Engine = ratings[1];
        Reliability = ratings[2];
        TireManagement = ratings[3];
        CalculateOverall();
    }

    public uint Aero { get; }
    public uint Engine { get; }
    public uint Reliability { get; }
    public uint TireManagement { get; }
    public uint Overall { get; private set; }

    private void CalculateOverall()
    {
        Overall = (Aero + Engine + Reliability + TireManagement) / 4;
    }
}