class A {
  StreamSubscription _subscriptionA; // LINT
  void init(Stream stream) {
    _subscriptionA = stream.listen((_) {});
  }
}
