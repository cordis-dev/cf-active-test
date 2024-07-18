class Person {
  final String? name;

  @override
  operator ==(Object? other) =>
      other is Person && name == other.name;
}
