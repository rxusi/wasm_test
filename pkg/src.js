import initSync, { greet, Tictactoe, Point } from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

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

//-----

let ttt;

const createBoard = function() {
    let cell_n = document.querySelector("#cell_n").value; 

    if (cell_n <= 0) { return; }

    let board = document.querySelector("#board");
    
    ttt = Tictactoe.new(cell_n, cell_n, 1);

    let boardHTML = ttt.getBoardHTML();

    board.innerHTML = boardHTML;
}
window.createBoard = createBoard;
export { createBoard };

const onCellClick = function(cell_id) {
    let y, x;

    [y, x] = idParse(cell_id);

    console.log(y, x);

    put(cell_id, y, x);
}
window.onCellClick = onCellClick;
export { onCellClick };

const put = function(cell_id, y, x) {
    let updated_cell_id;
    let uy, ux;
    
    // Player turn
    updated_cell_id = ttt.put(0, y, x);

    if (updated_cell_id == "") { 
        alert("");
        return;
    }

    [uy, ux] = idParse(updated_cell_id);
    document.querySelector("#" + cell_id).value = ttt.getCellStr(uy, ux);

    // Com turn
    updated_cell_id = ttt.put(0, 0, 0);
    [uy, ux] = idParse(updated_cell_id);
    document.querySelector("#" + cell_id).value = ttt.getCellStr(uy, ux);
}

const idParse = function(id_string) {
    let yx = id_string.split('_');

    let y = parseInt(yx[0]);
    let x = parseInt(yx[1]);

    return [y, x];
}