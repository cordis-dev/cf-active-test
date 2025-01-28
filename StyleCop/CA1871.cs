namespace App;

internal class CA1871
{
    public static void Print(int? value)
    {
        ArgumentNullException.ThrowIfNull(value);
        Console.WriteLine(value.Value);
    }
}
