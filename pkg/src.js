import initSync, {greet, Tictactoe} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

let ttt;

const create = function() {
    ttt = Tictactoe.new(4, 4);
}
window.create = create;
export {create};

const show = function() {
    ttt.show();
}
window.show = show;
export {show};