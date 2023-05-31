#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
mod random_generator; use random_generator::Rng;
mod montecarlo; use montecarlo::montecarlo;

#[wasm_bindgen]
pub struct Point {
    x: isize,
    y: isize,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: isize, y: isize) -> Self {
        return Point { x: x, y: y };
    }

    pub fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }

    pub fn sum(&self) -> isize {
        return self.x + self.y;
    }

    pub fn set(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
}

#[wasm_bindgen]
pub struct Tictactoe {
    N: usize,
    win_N: usize,
    remain_N: usize,
    turn: isize,
    winner: isize,
    board: Vec<Vec<isize>>,
    id_list: Vec<Vec<String>>,
    rng: Rng,
}

#[wasm_bindgen]
impl Tictactoe {
    #[wasm_bindgen(constructor)]
    pub fn new(_N: usize, _win_N: usize, first: isize) -> Self {
        let mut _remain_N: usize = _N * _N;
        let mut _board: Vec<Vec<isize>> = vec![vec![0; _N]; _N];
        let mut _id_list: Vec<Vec<String>> = vec![vec![String::new(); _N]; _N];
        
        for y in 0.._N {
            for x in 0.._N {
                _id_list[y][x] = String::from("_") + &y.to_string() + "_" + &x.to_string();
            }
        }

        /*_board[_N - 1][_N - 1] =  1;
        _board[_N - 0][_N - 0] =  1;
        _board[_N - 0][_N - 1] = -1;
        _board[_N - 1][_N - 0] = -1;*/

        let mut _rng: Rng = Rng::new(1000000007);

        let ttt: Self = Self { 
            N: _N, 
            win_N: _win_N,
            remain_N: _remain_N,
            turn: first,
            winner: 0,
            board: _board,
            id_list: _id_list,
            rng: _rng,
        };

        return ttt;
    }

    pub fn clone(&mut self) -> Self {
        let ttt: Self = Self {
            N: self.N,
            win_N: self.win_N,
            remain_N: self.remain_N,
            turn: self.turn,
            winner: self.winner,
            board: self.board.clone(),
            id_list: self.id_list.clone(),
            rng: Rng::new(self.rng.rand_usize(1, 99991) as u128),
        };

        return ttt;
    }

    fn getNextPos(&self, y: usize, x: usize, dx: isize, dy: isize, i: usize) -> Option<(usize, usize)> {
        let ret: Option<(usize, usize)>;

        let mul: isize = i as isize;

        if (x as isize + dx * mul < 0 || (self.N) as isize <= x as isize + dx * mul) || 
           (y as isize + dy * mul < 0 || (self.N) as isize <= y as isize + dy * mul) {
            ret = None;
        }
        else {
            ret = Some(((y as isize + dy * mul) as usize, (x as isize + dx * mul) as usize));
        }

        return ret;
    }

    fn getRandomPos(&mut self) -> (usize, usize) {
        let (mut x, mut y): (usize, usize);
        loop {
            y = self.rng.rand_usize(0, self.N);
            x = self.rng.rand_usize(0, self.N);

            if self.board[y][x] == 0 { break; }
        }

        return (y, x);
    }

    pub fn put(&mut self, stone: isize, _y: usize, _x: usize, strategy: &str) -> String {
        if self.winner != 0 || self.remain_N == 0 { return String::from("!!! The game is over !!!"); }

        let (y, x): (usize, usize);

        if stone != self.turn { return String::from("!!! It's not your turn !!!"); }

        if _x < self.N && _y < self.N { (y, x) = (_y, _x); }
        else { 
            if strategy == "random" { (y, x) = self.getRandomPos(); }
            else if strategy == "montecarlo" { (y, x) = montecarlo(self, stone, 500); }
            else { (y, x) = self.getRandomPos(); }
        }

        if self.board[y][x] != 0 { return String::from("!!! There already is a stone !!!"); }

        self.board[y][x] = stone;

        self.judge(y, x);

        self.turn = self.turn % 2 + 1;

        self.remain_N -= 1;

        return self.id_list[y][x].clone();
    }

    fn judge(&mut self, y: usize, x: usize) {
        const DX: [isize; 8] = [ 0, 1, 1, 1, 0,-1,-1,-1];
        const DY: [isize; 8] = [-1,-1, 0, 1, 1, 1, 0,-1];

        let mut stone_n_dir: [usize; 4] = [1, 1, 1, 1];
        let stone: isize = self.board[y][x];

        for dir in 0..8usize {
            for i in 1..self.N {
                if let Some((ny, nx)) = self.getNextPos(y, x, DY[dir], DX[dir], i) {
                    if self.board[ny][nx] == stone { stone_n_dir[dir % 4] += 1; }
                    else { break; }
                }
                else { break; }
            }
        }

        for stone_n in stone_n_dir {
            if stone_n >= self.win_N { self.winner = stone; }
        }
    }

    pub fn getCellStr(&self, y: usize, x: usize) -> String {
        let cell_str: String = Self::cell2Str(self.board[y][x]);
        return cell_str;
    }

    #[wasm_bindgen(js_name = "cell2Str")]
    pub fn cell2Str(v: isize) -> String {
        if v == 1 { return String::from("●"); }
        else if v == 2 { return String::from("○"); }
        else { return String::from(" "); }
    }

    fn getCellHTML(&self, y: usize, x: usize) -> String {
        let mut cellHTML: String = String::new();

        cellHTML += "<input ";
        cellHTML += &String::from("id='");
        cellHTML += &self.id_list[y][x];
        cellHTML += "' ";
        cellHTML += "type='button' class='cell' value='";
        cellHTML += &self.getCellStr(y, x);
        cellHTML += "' onClick='onCellClick(this.id)'>";

        return cellHTML;
    }

    pub fn show(&self) {
        for y in 0..self.N {
            for x in 0..self.N {
                print!("{} ", self.getCellStr(y, x));
            }
            println!();
        }
        println!();
    }

    pub fn getBoardHTML(&self) -> String {
        let mut boardHTML: String = String::new();

        let spanBegin: &str = "<span class='line'>";
        let spanEnd: &str = "</span>";

        for y in 0..self.N {
            boardHTML += spanBegin;

            for x in 0..self.N {
                boardHTML += &self.getCellHTML(y, x);
            }
            
            boardHTML += spanEnd;
        }

        return boardHTML;
    }

    pub fn getWinner(&self) -> isize {
        //if self.winner != 0 { println!("{} wins!", Self::cell2Str(self.winner)); }
        return self.winner;
    }
}

#[cfg(test)]
mod tests {
    use crate::Tictactoe;
    use crate::Rng;
    use crate::montecarlo;

    #[test]
    fn ttt_show() {
        let mut ttt: Tictactoe = Tictactoe::new(4, 4, 1);
        ttt.show();

        ttt.put(1, 0, 0, "random");
        ttt.show();
        ttt.put(2, 5, 5, "random");
        ttt.show();
        ttt.put(1, 0, 2, "random");
        ttt.show();
        ttt.put(2, 5, 5, "random");
        ttt.show();
        ttt.put(1, 0, 3, "random");
        ttt.show();
        ttt.put(2, 5, 5, "random");
        ttt.show();
        ttt.put(1, 0, 1, "random");
        ttt.show();
        ttt.put(2, 5, 5, "random");
        ttt.show();
        ttt.put(1, 5, 5, "random");
        ttt.show();

        println!("{}", ttt.getWinner());

        dbg!(ttt.getBoardHTML());
    }

    #[test]
    fn motecarlo() {
        let mut ttt: Tictactoe = Tictactoe::new(4, 4, 1);
        ttt.put(1, 0, 0, "random");
        ttt.show();
        ttt.put(2, 5, 5, "random");
        ttt.show();

        montecarlo(&mut ttt, 1, 1000);
    }

    #[test]
    fn rand_test() {
        let mut rng = Rng::new(1000000007);

        for _ in 0..1000 {
            println!("{}", rng.rand_usize(0, 100));
        }
    }
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}