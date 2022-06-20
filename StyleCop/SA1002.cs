public class TestClass
{
    public unsafe void TestMethod()
    {
        int* data = stackalloc int[] { 1, 1 } ;
    }
}
