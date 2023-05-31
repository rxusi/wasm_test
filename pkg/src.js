import initSync, {greet, Tictactoe} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

let ttt;

function create() {
    ttt = Tictactoe.new(4, 4);
}

function show() {
    ttt.show();
}