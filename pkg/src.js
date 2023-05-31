import initSync, {greet, Tictactoe, Point} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

let ttt;
let point;

const create = function() {
    point = new Point(2, 3);
    //ttt = Tictactoe.new(4, 4);
}
window.create = create;
export {create};

const show = function() {
    point.show();
    //ttt.show();
}
window.show = show;
export {show};

const sum = function() {
    let _sum = point.sum();
    console.log(_sum);
}
window.sum = sum;
export {sum};