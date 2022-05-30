/*eslint getter-return: "error"*/

p = {
    get name(){
        // no returns.
    }
};

p = {
    get name(){
        return "nicholas";
    }
};
