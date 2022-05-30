/*eslint no-unsafe-optional-chaining: "error"*/

var obj = undefined;

(obj?.foo)();

(obj?.foo)?.();
