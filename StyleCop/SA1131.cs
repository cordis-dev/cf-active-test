public class SA1131
{
  public void Method(string value)
  {
      if (null == value)
      {
          throw new ArgumentNullException(nameof(value));
      }
  }
}
