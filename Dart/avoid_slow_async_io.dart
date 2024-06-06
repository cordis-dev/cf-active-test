import 'dart:io';

Future<Null> someFunction() async {
  var file = File('/path/to/my/file');
  var now = DateTime.now();
  if ((await file.lastModified()).isBefore(now)) print('before'); // LINT
}
