import initSync, {greet} from "../pkg/wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});