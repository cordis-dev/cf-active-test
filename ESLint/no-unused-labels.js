A: var foo = 0;

B: {
    if (foo()) {
        break B;
    }
    bar();
}
