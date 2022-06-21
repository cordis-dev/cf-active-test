public class EmptyStatement {
  public void foo() {
    int i = 5;
    if (i > 3); // violation, ";" right after if statement
      i++;
  }
}
