/*eslint no-const-assign: "off"*/

with (point) {
    r = Math.sqrt(x * x + y * y); // is r a member of point?
}

const r = ({x, y}) => Math.sqrt(x * x + y * y);
console.log(r);
