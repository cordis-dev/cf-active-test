let foo = function() {
    try {
        return 1;
    } finally {
        return 3;
    }
};

let foo2 = function() {
    try {
        return 1;
    } finally {
        console.log("hola!");
    }
};

console.log(foo);
console.log(foo2);
