/*eslint no-dupe-keys: "error"*/

var foo = {
    bar: "baz",
    bar: "qux"
};

var foo2 = {
    bar: "baz",
    quxx: "qux"
};

console.log(foo);
console.log(foo2);
