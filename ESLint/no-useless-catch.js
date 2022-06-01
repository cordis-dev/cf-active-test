try {
  doSomethingThatMightThrow();
} catch (e) {
  throw e;
}

try {
  doSomethingThatMightThrow();
} catch (e) {
  doSomethingBeforeRethrow();
  throw e;
}
