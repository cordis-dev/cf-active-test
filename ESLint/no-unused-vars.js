// global unused
var global_var = 42;

// after-used - unused positional arguments that occur before the last used argument will not be checked, but all named arguments and all positional arguments after the last used argument will be checked.
(function(foo, bar, baz, qux) {
    return qux;
})();


// caughtErrors
try {
    //...
} catch (err) {
    console.error("errors");
}

// By default, unused arguments cause warnings.
(function(arg1) {
    return 5;
})();
