using Spectre.Console;

public class Presenter : IPresenter
{
    public void Display(string message)
    {
        AnsiConsole.Markup(message + "\n");
    }
}