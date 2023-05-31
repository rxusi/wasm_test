import initSync, {greet, Tictactoe, Point} from "./wasm_test.js";
    
initSync()
.then(() => {
    greet("World")
});

let ttt;
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

const createBoard = function() {
    let cell_n = document.querySelector("#cell_n").value; 
    let board = document.querySelector("#board");
    
    /*console.log(cell_n);

    let boardHTML = "";

    for (let h = 0; h < cell_n * 2; h++) {
        boardHTML += "<span class='line'>";

        for (let w = 0; w < cell_n * 2; w++) {
            boardHTML += "<input type='button' class='cell' value='○'>";
        }

        boardHTML += "</span>";
    }
    
    board.innerHTML = boardHTML;
    */

    ttt = Tictactoe.new(cell_n, cell_n);

    let boardHTML = ttt.getBoardHTML();

    board.innerHTML = boardHTML;
}
window.createBoard = createBoard;
export {createBoard};