import initSync, { Tictactoe } from "./wasm_test.js";

const PLAYER = 1;
const COM = 2;

let cell_n;
let ttt;
let winner;

const createBoard = function() {
    cell_n = document.querySelector("#cell_n").value; 
    let win_n = document.querySelector("#win_n").value; 

    if (cell_n <= 0) { return; }

    let board = document.querySelector("#board");
    
    ttt = new Tictactoe(cell_n, win_n, 1);
    winner = 0;

    let boardHTML = ttt.getBoardHTML();

    board.innerHTML = boardHTML;

    document.querySelector("#winner").innerHTML = "";

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
    displayWinner();
    
    // Com turn
    put(COM, cell_n, cell_n);
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
    if (winner == 0) {
        winner = ttt.getWinner();
        let winner_text = "";

        if (winner == PLAYER) { winner_text = "Player "; }
        else if (winner == COM ) { winner_text = "Com "; }

        winner_text += Tictactoe.cell2Str(winner) + " wins!";

        if (winner != 0) {
            alert(winner_text);
            document.querySelector("#winner").innerHTML = winner_text;
        }
    }
}

const idParse = function(id_string) {
    let yx = id_string.split('_');

    let y = parseInt(yx[1]);
    let x = parseInt(yx[2]);

    return [y, x];
}