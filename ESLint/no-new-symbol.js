/*eslint no-new-symbol: "error"*/
/*eslint-env es6*/

var foo = new Symbol('foo');

var foo2 = Symbol('foo');

console.log(foo);
console.log(foo2);
