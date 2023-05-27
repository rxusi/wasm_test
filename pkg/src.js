import initSync, {greet} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});