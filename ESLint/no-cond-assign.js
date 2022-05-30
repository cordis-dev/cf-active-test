/*eslint no-cond-assign: "error"*/

// Unintentional assignment
var x;
if (x = 0) {
    var b = 1;
}

// Assignment replaced by comparison
if (x === 0) {
    var b = 1;
}
