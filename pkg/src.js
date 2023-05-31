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

const PLAYER = 1;
const COM = 2;

let cell_n;
let ttt;
let winner;

const createBoard = function() {
    cell_n = document.querySelector("#cell_n").value; 

    if (cell_n <= 0) { return; }

    let board = document.querySelector("#board");
    
    ttt = new Tictactoe(cell_n, cell_n, 1);
    winner = 0;

    let boardHTML = ttt.getBoardHTML();

    board.innerHTML = boardHTML;

    // Com tries to put
    put(COM, cell_n, cell_n);
}
window.createBoard = createBoard;
export { createBoard };

const onCellClick = function(cell_id) {
    let y, x;

    [y, x] = idParse(cell_id);

    console.log(y, x);

    // Player turn
    put(PLAYER, y, x);
    winner = ttt.gameIsOver();
    
    // Com turn
    put(COM, cell_n, cell_n);
    winner = ttt.gameIsOver();

    displayWinner();
}
window.onCellClick = onCellClick;
export { onCellClick };

const put = function(stone, y, x) {
    if (winner != 0) { return; }

    let updated_cell_id;
    let uy, ux;
    
    updated_cell_id = ttt.put(stone, y, x);

    if (updated_cell_id[0] == '!') { 
        console.log(updated_cell_id);
        return;
    }

    [uy, ux] = idParse(updated_cell_id);
    document.querySelector("#" + updated_cell_id).value = ttt.getCellStr(uy, ux);
}

const displayWinner = function() {
    let winner = ttt.getWinner();
    let winner_text = "";

    if (winner == PLAYER) { winner_text = "Player"; }
    else if (winner == COM ) { winner_text = "Com"; }

    winner_text += Tictactoe.cell2Str(winner) + " wins!";

    alert(winner_text);
}

const idParse = function(id_string) {
    let yx = id_string.split('_');

    let y = parseInt(yx[1]);
    let x = parseInt(yx[2]);

    return [y, x];
}