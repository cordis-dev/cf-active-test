public class EmptyBlock {
  private void t() {
    try { // violation

    } catch (Exception e) {
      // ignored
    }
  }
}
