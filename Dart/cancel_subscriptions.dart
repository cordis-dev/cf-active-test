import 'dart:async';
void f(Stream stream) {
  StreamSubscription s = stream.listen((_) {});
}
