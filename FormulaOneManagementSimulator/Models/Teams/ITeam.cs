
public interface ITeam
{
    string Name { get; }
    string DriverOneName { get; }
    string DriverTwoName { get; }
    ITeamRating TeamRating { get; }
    ICarRating CarRating { get; }
    IPoints Points { get; }

    void AddPoints(uint points);
    void Display(IPresenter presenter);
}