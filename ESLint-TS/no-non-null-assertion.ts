interface Foo {
  bar?: string;
}

const foo: Foo = getFoo();
const includesBaz: boolean = foo.bar!.includes('baz');
