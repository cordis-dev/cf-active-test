/*eslint no-empty-character-class: "error"*/

/^abc[]/.test("abcdefg"); // false

/^abc/.test("abcdefg"); // true
