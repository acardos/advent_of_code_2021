use std::{fs, vec, cmp::{Ordering, min_by}, usize};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let random_numbers: Vec<u32> = lines.next().expect("Invalid input").split(",").map(|n| n.parse::<u32>().expect("Invalid input in random numbers")).collect();
    lines.next();

    let mut boards: Vec<Vec<Vec<u32>>> = vec![vec![]];

    loop {
        let maybe_line = lines.next();
        match maybe_line {
            Some(line) => {
                println!("{}", line);
                if line.trim().is_empty() {
                    boards.push(vec![]);
                } else {
                    let line_numbers = line.split(" ").filter(|l| !l.is_empty()).map(|n| n.parse::<u32>().expect("Invalid input in board")).collect();
                    boards.last_mut().unwrap().push(line_numbers);
                }
            }
            None => {
                break;
            }
        }
    }

    println!("{:?}", random_numbers);
    println!("{:?}", boards);

    let mut scores = vec![];
    for board in boards[..].iter() {
        let marked_board = create_marked_board(&board, &random_numbers);
        // println!("{:?}", marked_board);

        let (row_winnigs, column_winnings) = find_winning_rows_columns(&marked_board);
        // println!("{:?}", row_winnigs);
        // println!("{:?}", column_winnings);

        let min_row_winnig = find_min_winning(&row_winnigs);
        let min_col_winnig = find_min_winning(&column_winnings);

        let (min_winning_n, min_winning_idx) = min_by(min_row_winnig, min_col_winnig, idx_option_comparator);
        println!("min_winning_n: {:?}", min_winning_n);
        println!("min_winning_idx: {:?}", min_winning_idx);

        let unmarked = get_unmarked_until(&marked_board, min_winning_idx.unwrap());
        let score: u32 = unmarked.iter().sum::<u32>() * min_winning_n;
        println!("score: {}", score);
        scores.push((min_winning_idx.unwrap(), score));
    }

    let winning_score = scores.iter().min_by_key(|(idx, _score)| idx);
    let max_score_by_score = scores.iter().max_by_key(|(_idx, score)| score);
    println!("{:?}", winning_score);
    println!("{:?}", max_score_by_score);
}

fn create_marked_board(board: &Vec<Vec<u32>>, numbers: &Vec<u32>) -> Vec<Vec<(u32, Option<usize>)>> {
    let mut marked_board = vec![];

    for row in board {
        let marked_row = row.iter().map(|&n| (n, index(numbers, n))).collect();
        marked_board.push(marked_row);
    }

    marked_board
}

fn index(vector: &Vec<u32>, number: u32) -> Option<usize> {
    vector.iter().position(|&x| x == number)
}

fn find_winning_rows_columns(marked_board: &Vec<Vec<(u32, Option<usize>)>>) -> (Vec<&(u32, Option<usize>)>, Vec<&(u32, Option<usize>)>) {
    let winning_rows: Vec<&(u32, Option<usize>)> = marked_board.iter().map(|row| {
        let roww = row.iter().map(|e| e).collect();
        find_winning(&roww)
    }).collect();
    
    let columns: Vec<Vec<&(u32, Option<usize>)>> = (0..marked_board[0].len()).map(|i| marked_board.iter().map(|r| &r[i]).collect()).collect();
    let winning_columns: Vec<&(u32, Option<usize>)> = columns.iter().map(|column| find_winning(column)).collect();
    
    (winning_rows, winning_columns)
}

fn find_winning<'a>(v: &Vec<&'a(u32, Option<usize>)>) -> &'a(u32, Option<usize>) {
    v.iter().max_by(|x, y| idx_option_comparator(x, y)).unwrap()
}

fn find_min_winning<'a>(winnings: &Vec<&'a(u32, Option<usize>)>) -> &'a(u32, Option<usize>) {
    winnings.iter().min_by(|x, y| idx_option_comparator(x, y)).unwrap()
}

fn idx_option_comparator((_x_n, x_index_option): &&(u32, Option<usize>), (_y_n, y_index_option): &&(u32, Option<usize>)) -> Ordering {
    match x_index_option {
        None => Ordering::Less,
        Some(x_index) => {
            match y_index_option {
                None => Ordering::Greater,
                Some(y_index) => x_index.cmp(&y_index)
            }
        }
    }
}

fn get_unmarked_until(marked_board: & Vec<Vec<(u32, Option<usize>)>>, max: usize) -> Vec<u32> {
    let mut entries = vec![];
    for row in marked_board {
        for (n, maybe_idx) in row {
            match maybe_idx {
                None => entries.push(*n),
                Some(idx) => {
                    if idx > &max {
                        entries.push(*n);
                    }
                }
            }
        }
    }

    return entries;
}