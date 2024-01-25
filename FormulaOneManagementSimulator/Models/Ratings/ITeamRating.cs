public interface ITeamRating
{
    uint InSeasonDevelopment { get; }
    uint Repairs { get; }
    uint RaceSetup { get; }
    uint Strategy { get; }
    uint Management { get; }
    uint Overall { get; }
}