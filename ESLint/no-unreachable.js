function foo() {
    return true;
    console.log("done");
}

function foo2() {
    return bar();
    function bar() {
        return 1;
    }
}

foo();
foo2();
