public interface IRandomGenerator
{
    uint Generate(int seed);
    uint[] Generate(int[] seeds);
    int[] GenerateSeeds(uint numberOfSeeds);
}