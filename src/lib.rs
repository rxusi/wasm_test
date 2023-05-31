#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
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
    winner: isize,
    board: Vec<Vec<isize>>,
}

impl Tictactoe {
    const DX: [isize; 8] = [ 0, 1, 1, 1, 0,-1,-1,-1];
    const DY: [isize; 8] = [-1,-1, 0, 1, 1, 1, 0,-1];

    pub fn new(_N: usize, _win_N: usize) -> Self {
        let mut _board: Vec<Vec<isize>> = vec![vec![0; _N * 2]; _N * 2];
        
        _board[_N - 1][_N - 1] =  1;
        _board[_N - 0][_N - 0] =  1;
        _board[_N - 0][_N - 1] = -1;
        _board[_N - 1][_N - 0] = -1;

        let ttt: Self = Self { 
            N: _N, 
            win_N: _win_N,
            winner: 0,
            board: _board,
        };

        return ttt;
    }

    fn getNextPos(&self, x: usize, y: usize, dx: isize, dy: isize, i: usize) -> Option<(usize, usize)> {
        let ret: Option<(usize, usize)>;

        let mul: isize = i as isize;

        if (x as isize + dx * mul < 0 || (self.N * 2) as isize <= x as isize + dx * mul) || 
           (y as isize + dy * mul < 0 || (self.N * 2) as isize <= y as isize + dy * mul) {
            ret = None;
        }
        else {
            ret = Some(((x as isize + dx * mul) as usize, (y as isize + dy * mul) as usize));
        }

        return ret;
    }

    pub fn put(&mut self, stone: isize, x: usize, y: usize) -> bool {
        if self.board[y][x] != 0 { return false; }

        self.board[y][x] = stone;

        let mut stone_n_dir: [usize; 4] = [1, 1, 1, 1];

        for dir in 0..8usize {
            for i in 1..self.N * 2 {
                if let Some((nx, ny)) = self.getNextPos(x, y, Self::DX[dir], Self::DY[dir], i) {
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

        return true;
    }

    fn getChar(v: isize) -> char {
        if v > 0 { return '●'; }
        else if v < 0 { return '○'; }
        else { return '□'; }
    }

    pub fn show(&self) {
        for i in 0..self.N * 2 {
            for j in 0..self.N * 2 {
                print!("{} ", Tictactoe::getChar(self.board[i][j]));
            }
            println!();
        }
        println!();
    }

    pub fn gameIsOver(&self) -> bool {
        if self.winner != 0 {
            println!("{} wins!", Self::getChar(self.winner));
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

    #[test]
    fn ttt_show() {
        let mut ttt: Tictactoe = Tictactoe::new(4, 4);
        ttt.show();

        ttt.put(1, 2, 2);
        ttt.show();
        ttt.put(1, 5, 5);
        ttt.show();

        ttt.gameIsOver();
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