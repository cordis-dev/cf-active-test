namespace App;

internal class CA2023
{
    private readonly ILogger _logger;

    public CA2023(ILogger<Example> logger)
    {
        _logger = logger;
    }

    public void LogData(string name, int value)
    {
        // Violation: unmatched opening brace.
        _logger.LogInformation("Processing {Name with value {Value}", name, value);

        // Violation: unmatched closing brace.
        _logger.LogInformation("Processing Name} with value {Value}", name, value);
    }
}
