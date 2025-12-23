using System.IO;
using System.Threading.Tasks;

namespace StyleCopAnalyzers.Tests
{
    public class CA2024
    {
        public async Task Example(StreamReader streamReader)
        {
            while (!streamReader.EndOfStream)
            {
                string? line = await streamReader.ReadLineAsync();
                // Do something with line.
            }
        }
    }
}
