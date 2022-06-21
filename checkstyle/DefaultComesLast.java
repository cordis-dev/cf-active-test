public class DefaultComesLast {
  public void t()
  {
    switch (i) {
      case 1:
        break;
      default: // violation, 'default' before 'case'
        break;
      case 2:
        break;
    }
  }
}
