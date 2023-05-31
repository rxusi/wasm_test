import initSync, {greet, Tictactoe} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

let ttt;

export function create() {
    ttt = Tictactoe.new(4, 4);
}

export function show() {
    ttt.show();
}