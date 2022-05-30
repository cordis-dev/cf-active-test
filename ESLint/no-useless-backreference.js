/*eslint no-useless-backreference: "error"*/

/^(?:(a)|\1b)$/; // reference to (a) into another alternative

/^(?:(a)|(b)\2)$/; // reference to (b)
