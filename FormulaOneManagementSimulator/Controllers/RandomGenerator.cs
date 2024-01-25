public class RandomGenerator : IRandomGenerator
{
    public uint Generate(int seed)
    {
        Random random = new(seed);

        return (uint)random.Next(50, 99);
    }

    public uint[] Generate(int[] seeds)
    {
        List<uint> randomNumbers = new();

        for (int i = 0; i < seeds.Length; i++)
        {
            Random random = new(seeds[i]);

            randomNumbers.Add((uint)random.Next(50, 99));
        }

        return randomNumbers.ToArray();
    }

    public int[] GenerateSeeds(uint numberOfSeeds)
    {
        List<int> seeds = new();

        for (int i = 0; i < numberOfSeeds; i++)
        {
            Random random = new();

            seeds.Add(random.Next());
        }

        return seeds.ToArray();
    }
}