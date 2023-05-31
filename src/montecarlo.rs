#![allow(non_snake_case)]

use super::Tictactoe;

pub fn montecarlo(_ttt: &mut Tictactoe, stone: isize, tryout_n: isize) -> (usize, usize) {
    let mut base: Tictactoe = _ttt.clone();
    println!("{:?}", &base.board);

    let (mut ny, mut nx): (usize, usize) = (0, 0);

    let mut sim_board: Vec<Vec<isize>> = vec![vec![0; base.N]; base.N];

    for y in 0..base.N {
        for x in 0..base.N {
            for n in 0..tryout_n {
                let mut ttt_try: Tictactoe = base.clone();
                let put_text: String = ttt_try.put(stone, y, x, "random");
                
                if put_text.find('!') != None { 
                    sim_board[y][x] = std::isize::MIN;
                    break; 
                }
                
                for turn in 0..ttt_try.remain_N as isize {
                    ttt_try.put(turn % 2 + 1, base.N, base.N, "random");
                    if ttt_try.getWinner() != 0 { break; }
                }

                if sim_board[y][x] + tryout_n - n <= sim_board[ny][nx] { break; }

                if ttt_try.getWinner() == stone { sim_board[y][x] += 1; }
                if ttt_try.getWinner() == stone % 2 + 1 { sim_board[y][x] -= 1; }
            }

            if sim_board[ny][nx] < sim_board[y][x] { (ny, nx) = (y, x); }
        }
    }

    dbg!("{:?}", &sim_board);

    return (ny, nx);
}