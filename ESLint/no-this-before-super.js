/*eslint no-this-before-super: "error"*/
/*eslint-env es6*/

class A extends B {
    constructor() {
        this.a = 0;
        super();
    }
}

class C {
    constructor() {
        this.a = 0; // OK, this class doesn't have an `extends` clause.
    }
}
