/*eslint-env es6*/

switch (foo) {
    case 1:
        let x = 1;
        console.log(x);
        break;
    default:
        //
}

switch (foo) {
    // The following case clauses are wrapped into blocks using brackets
    case 1: {
        let x = 1;
        console.log(x);
        break;
    }
    default: {
        //
    }
}
