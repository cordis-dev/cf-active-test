namespace ConsoleApp10
{
    internal class CA2019
    {
        [ThreadStatic]
        private static Object obj = new();
    }
}
