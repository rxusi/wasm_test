#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
mod random_generator; use random_generator::Rng;

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
    turn: isize,
    winner: isize,
    board: Vec<Vec<isize>>,
    id_list: Vec<Vec<String>>,
    rng: Rng,
}

#[wasm_bindgen]
impl Tictactoe {
    pub fn new(_N: usize, _win_N: usize, first: isize) -> Self {
        let mut _board: Vec<Vec<isize>> = vec![vec![0; _N]; _N];
        let mut _id_list: Vec<Vec<String>> = vec![vec![String::new(); _N]; _N];
        
        for y in 0.._N {
            for x in 0.._N {
                _id_list[y][x] = String::from("id='") + &y.to_string() + "_" + &x.to_string() + "' ";
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
            turn: first,
            winner: 0,
            board: _board,
            id_list: _id_list,
            rng: _rng,
        };

        return ttt;
    }

    fn getNextPos(&self, x: usize, y: usize, dx: isize, dy: isize, i: usize) -> Option<(usize, usize)> {
        let ret: Option<(usize, usize)>;

        let mul: isize = i as isize;

        if (x as isize + dx * mul < 0 || (self.N) as isize <= x as isize + dx * mul) || 
           (y as isize + dy * mul < 0 || (self.N) as isize <= y as isize + dy * mul) {
            ret = None;
        }
        else {
            ret = Some(((x as isize + dx * mul) as usize, (y as isize + dy * mul) as usize));
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

    pub fn put(&mut self, _stone: isize, _x: usize, _y: usize) -> bool {
        let stone: isize;
        let (mut y, mut x): (usize, usize) = (self.N, self.N);

        if _stone == 0 {
            stone = self.turn;
            if stone == -1 { (x, y) = self.getRandomPos(); }
        }
        else {
            stone = _stone;
        }

        if x == self.N && y == self.N {
            if _x < self.N && _y < self.N { (x, y) = (_x, _y); }
            else { (x, y) = self.getRandomPos(); }
        }

        if self.board[y][x] != 0 { return false; }

        const DX: [isize; 8] = [ 0, 1, 1, 1, 0,-1,-1,-1];
        const DY: [isize; 8] = [-1,-1, 0, 1, 1, 1, 0,-1];

        self.board[y][x] = stone;

        let mut stone_n_dir: [usize; 4] = [1, 1, 1, 1];

        for dir in 0..8usize {
            for i in 1..self.N {
                if let Some((nx, ny)) = self.getNextPos(x, y, DX[dir], DY[dir], i) {
                    if self.board[ny][nx] == stone { stone_n_dir[dir % 4] += 1; }
                    else { break; }
                }
                else { break; }
            }
        }

        dbg!(&stone_n_dir);

        for stone_n in stone_n_dir {
            if stone_n >= self.win_N { self.winner = stone; }
        }

        if _stone == 0 { self.turn *= -1; }

        return true;
    }

    pub fn getCellStr(&self, y: usize, x: usize) -> String {
        let cell_str: String = Self::cell2Str(self.board[y][x]);
        return cell_str;
    }

    fn cell2Str(v: isize) -> String {
        if v > 0 { return String::from("●"); }
        else if v < 0 { return String::from("○"); }
        else { return String::from(" "); }
    }

    fn getCellHTML(&self, y: usize, x: usize) -> String {
        let mut cellHTML: String = String::new();

        cellHTML += "<input ";
        cellHTML += &self.id_list[y][x];
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

    pub fn gameIsOver(&self) -> bool {
        if self.winner != 0 {
            println!("{} wins!", Self::cell2Str(self.winner));
            return true;
        }
        else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tictactoe;
    use crate::Rng;

    #[test]
    fn ttt_show() {
        let mut ttt: Tictactoe = Tictactoe::new(4, 4, 1);
        ttt.show();

        ttt.put(0, 2, 2);
        ttt.show();
        ttt.put(0, 5, 5);
        ttt.show();
        ttt.put(0, 1, 1);
        ttt.show();

        ttt.gameIsOver();

        dbg!(ttt.getBoardHTML());
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