public interface IQuery
{
    ITeam FindTeam(string teamName);
    void SetTeams(ITeam[] teams);
}