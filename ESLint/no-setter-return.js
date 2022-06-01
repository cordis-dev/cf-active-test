var foo = {
    set a(value) {
        this.val = value;
        return value;
    }
};

var foo2 = {
    set a(value) {
        this.val = value;
    }
};

console.log(foo);
console.log(foo2);
