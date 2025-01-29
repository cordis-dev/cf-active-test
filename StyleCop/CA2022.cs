namespace App;

internal class CA2022
{
    void M1(Stream stream, byte[] buffer)
    {
        // CA2022 violation.
        stream.Read(buffer, 0, buffer.Length);

        // Fix for the violation.
        stream.ReadExactly(buffer);
    }
}
