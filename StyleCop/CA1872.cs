using System.Text;

namespace App;

public class CA1872
{
    private readonly byte[] _data = Encoding.ASCII.GetBytes("Hello World");

    public string Encode()
    {
        return Convert.ToHexString(_data);
    }

    public string EncodeToLower()
    {
        return Convert.ToHexStringLower(_data);
    }
}