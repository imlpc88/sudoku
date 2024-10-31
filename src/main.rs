use std::env;
use std::fs;

fn main() {
    let mut mtx:[[u8; 9]; 9] = [[0; 9]; 9];
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
    for line in contents.split("\n") {
        for i in 0..81 {
            // println!("{}", &line[i..=i]);
            if "." != &line[i..=i] {
                let c: u8 = line[i..=i].parse().unwrap();
                mtx[i/9][i%9] = c;
            } else {
                mtx[i/9][i%9] = 0;
            }
        }
        solve_sudoku(mtx);
    }
 }

 fn print_sudoku(sudoku: [[u8; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", sudoku[i][j]);
        }
    }
    println!("");
 }

 fn find_possible_values(sudoku: [[u8; 9]; 9], i: usize, j: usize) -> Vec<u8> {
    let mut v: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
    for k in 0..9 {
        v.retain(|&x| x != sudoku[i][k]);
        v.retain(|&x| x != sudoku[k][j]);
    }
    for l in 0..3 {
        for m in 0..3 {
            v.retain(|&x | x != sudoku[i/3*3+l][j/3*3+m]);
        }
    }
    v
}

fn find_blank(sudoku: [[u8; 9]; 9]) -> (usize, usize) {
    for m in 0..9 {
        for n in 0..9 {
            if sudoku[m][n] == 0 {
                return (m, n);
            }
        }
    }
    (255, 255)
}

fn solve_sudoku(mut sudoku: [[u8; 9]; 9]) {
    let (row, col) = find_blank(sudoku);
    if (row, col) == (255, 255) {
        print_sudoku(sudoku);
        return ();
    } else {
        let values = find_possible_values(sudoku, row, col);
        for v in values {
            sudoku[row][col] = v;
            solve_sudoku(sudoku);
        }
    }
}