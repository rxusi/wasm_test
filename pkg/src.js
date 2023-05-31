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
    
    ttt = Tictactoe.new(cell_n, cell_n);

    let boardHTML = ttt.getBoardHTML();

    board.innerHTML = boardHTML;
}
window.createBoard = createBoard;
export { createBoard };

const onCellClick = function(cell_id) {
    let yx = cell_id.split('_');

    y = parseInt(yx[0]);
    x = parseInt(yx[1]);

    console.log(yx, y, x);
}
window.onCellClick = onCellClick;
export { onCellClick };