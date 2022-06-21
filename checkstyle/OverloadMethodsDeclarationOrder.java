public class OverloadMethodsDeclarationOrder {
  public void foo(int i) {} // OK
  public void foo(String s) {} // OK
  public void notFoo() {} // violation. Have to be after foo(String s, int i)
  public void foo(int i, String s) {}
  public void foo(String s, int i) {}
}
