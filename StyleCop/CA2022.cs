namespace App;

internal class CA2022
{
    void M1(Stream stream, byte[] buffer)
    {
        // CA2022 violation.
        stream.ReadExactly(buffer);

        // Fix for the violation.
        stream.ReadExactly(buffer);
    }
}
