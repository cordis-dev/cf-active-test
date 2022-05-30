/*eslint no-dupe-else-if: "error"*/

if (isSomething(x)) {
    foo();
} else if (isSomething(x)) {
    bar();
}

if (isSomething(x)) {
    foo();
} else if (isSomethingElse(x)) {
    bar();
}
