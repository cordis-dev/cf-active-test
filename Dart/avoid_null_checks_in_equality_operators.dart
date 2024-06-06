class Person {
  final String? name;

  @override
  operator ==(Object? other) =>
      other != null && other is Person && name == other.name;
}
