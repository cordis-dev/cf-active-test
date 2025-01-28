using System.Text;

namespace App;

public class CA1872
{
    private readonly byte[] _data = Encoding.ASCII.GetBytes("Hello World");

    public string Encode()
    {
        return BitConverter.ToString(_data).Replace("-", "");
    }

    public string EncodeToLower()
    {
        return BitConverter.ToString(_data).Replace("-", "").ToLower();
    }
}