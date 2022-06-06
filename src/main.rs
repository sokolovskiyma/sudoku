use std::{env, fs, process};
use sudoku::Sudoku;

fn main() {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Проблема с разбором аргументов");
            process::exit(1);
        }
    };

    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Проблема с чтением файла: {}", err);
        process::exit(1);
    });
    let mut sudoku_vec: Vec<Vec<u8>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<u8> = Vec::new();
        line.split("")
            .filter(|l| l != &"")
            .for_each(|l| {
                let digit = l.parse::<u8>().unwrap_or_else(|err| {
                    eprintln!("Проблема разбором числа: {}", err);
                    process::exit(1);
                });
                if digit > 9 {
                    eprintln!("Число > 9");
                    process::exit(1);
                }
                row.push(digit);
            });
        if row.len() != 9 {
            eprintln!("Непрвильная длинна строки");
            process::exit(1);
        }
        sudoku_vec.push(row);
    }

    if sudoku_vec.len() != 9 {
        eprintln!("Непрвильное количество строк");
        process::exit(1);
    }

    let sudoku: Sudoku = Sudoku::from_vec(sudoku_vec);

    println!("{}", sudoku.solve().unwrap());
}
