using Xunit;

public class RandomGeneratorShould
{
    [Theory]
    [InlineData(1, 62)]
    [InlineData(12, 98)]
    [InlineData(123, 98)]
    [InlineData(1234, 69)]
    public void GenerateRandomNumberFromASeed(int seed, uint expectedNumber)
    {
        // Given
        IRandomGenerator randomGenerator = new RandomGenerator();

        // When
        uint randomNumber = randomGenerator.Generate(seed);

        // Then
        Assert.Equal(expectedNumber, randomNumber);
    }

    [Theory]
    [InlineData(new int[] { 1, 2 }, new uint[] { 62, 87 })]
    [InlineData(new int[] { 3, 4, 5, 6 }, new uint[] { 64, 89, 66, 92 })]
    [InlineData(new int[] { 100, 200, 300, 400, 500, 600, 700 }, new uint[] { 97, 60, 72, 84, 96, 58, 70 })]
    public void GenerateRandomNumbersFromSeeds(int[] seeds, uint[] expectedNumbers)
    {
        // Given
        IRandomGenerator randomGenerator = new RandomGenerator();

        // When
        uint[] randomNumbers = randomGenerator.Generate(seeds);

        // Then
        Assert.Equal(expectedNumbers, randomNumbers);
    }

    [Theory]
    [InlineData(1)]
    [InlineData(5)]
    [InlineData(10)]
    public void GenerateSeeds(uint numberOfSeeds)
    {
        // Given
        IRandomGenerator randomGenerator = new RandomGenerator();

        // When
        int[] seeds = randomGenerator.GenerateSeeds(numberOfSeeds);

        // Then
        Assert.Equal((int)numberOfSeeds, seeds.Length);
    }
}