/*eslint constructor-super: "error"*/
/*eslint-env es6*/

class A extends B {
    constructor() { }  // Would throw a ReferenceError.
}
