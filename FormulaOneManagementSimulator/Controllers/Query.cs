public class Query : IQuery
{
    private ITeam[] teams;

    public Query(ITeam[] teams)
    {
        this.teams = teams;
    }

    public ITeam FindTeam(string teamName)
    {
        foreach (var team in from ITeam team in teams
                             where teamName == team.Name
                             select team)
        {
            return team;
        }

        throw new ArgumentOutOfRangeException("Driver can't find team");
    }

    public void SetTeams(ITeam[] teams)
    {
        this.teams = teams;
    }
}