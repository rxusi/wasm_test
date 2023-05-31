/*
const createBoard = function() {
    let cell_n = document.querySelector("#cell_n").value; 
    let board = document.querySelector("#board");
    console.log(cell_n);

    let boardHTML = "";

    let id_y, id_x;

    for (let h = 0; h < cell_n * 2; h++) {
        boardHTML += "<span class='line'>";

        id_y = String(h);

        for (let w = 0; w < cell_n * 2; w++) {
            id_yx = id_y + "_" + String(w);

            boardHTML += "<input type='button' id='" + id_yx + "' class='cell' value='○' onClick='onCellClick(this.id)'>";
        }

        boardHTML += "</span>";
    }
    
    board.innerHTML = boardHTML;

    cells = document.getElementsByClassName("cell");

    
}

const onCellClick = function(cell_id) {
    console.log(cell_id);

    yx = cell_id.split('_');

    y = parseInt(yx[0]);
    x = parseInt(yx[1]);

    console.log(yx, y, x);
}
*/