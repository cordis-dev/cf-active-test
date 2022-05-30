/*eslint no-unused-labels: "error"*/

A: var foo = 0;

B: {
    if (foo()) {
        break A;
    }
    bar();
}
