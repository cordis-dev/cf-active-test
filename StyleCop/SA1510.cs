public class SA1510
{
    public void Test()
    {
        try
        {
            SomeMethod();
        }

        catch (Exception ex)
        {
            Console.WriteLine(ex.ToString());
        }
    }

    public void SomeMethod()
    {
    }
}
