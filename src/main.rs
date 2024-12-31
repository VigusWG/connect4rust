use std::{io::{self, stdin, stdout, Write}, iter, result};

#[derive(Clone)]
#[derive(Copy)]

struct Column {
    len: u8,
    our_tokens: [bool; 6] // true ours false not
}

impl Column {
    fn add_to_column (&mut self, ours: bool){
        self.our_tokens[self.len as usize] = ours;
        self.len += 1;
    }

    fn remove_from_column (&mut self){
        self.len -= 1;
    }
}


fn ev_rows(&board: &[Column; 7]) -> i8{
    for i in 0..6{
        if board[4].len <= i {break;}
        let middle = board[3].our_tokens[usize::from(i)];
        
        if (4..7).take_while(|j| board[*j].len > i && middle == board[*j].our_tokens[usize::from(i)]).count() +  
            (0..3).rev().take_while(|j| board[*j].len > i && middle == board[*j].our_tokens[usize::from(i)]).count() >= 3{
            return if middle {1} else {-1}
        }
    }
    return 0;
}

fn ev_column(col: &Column) -> i8{
    if col.len < 4 { return 0;}
    for i in 0..(col.len-3) {
        if col.our_tokens.iter().skip(usize::from(i+1)).take(3).all(|x| *x == col.our_tokens[usize::from(i)]) {
            return if col.our_tokens[usize::from(i)] {1} else {-1}
        }
    }
    return 0;
}
fn ev_columns(board: &[Column; 7]) -> i8{
    for i in 0..7 {
        let a = ev_column(&board[i]);
        if a == 0 {continue;}
        return a;
    }
    return 0;
}

fn ev_diagonals(board: &[Column; 7]) -> i8 {
    let check_diag = |col: u8, row:u8, rev: bool | -> i8 {
        if board[usize::from(col)].len <= row {return 0;}
        let mut first = board[usize::from(col)].our_tokens[usize::from(row)];

        for i in 1u8..4{
            let next: Column = board[if rev {usize::from(col - i)} else {usize::from(col + i)}];
            let next_index = usize::from(row + i);
            if next.len <= i + row {return 0;}
            if next.our_tokens[next_index] != first {return 0;}
            first = next.our_tokens[next_index];
        }
        return if first {1} else {-1}
    };

    for col in 0..4 {
        for row in 0..3 {
            match check_diag(col, row, false){
                1 => return 1,
                -1 => return -1,
                _ => {}
            };
        }
    }

    for col in 3..7 {
        for row in 0..3 {
            match check_diag(col, row, true){
                1 => return 1,
                -1 => return -1,
                _ => {}
            };
        }
    }

    return 0;
}

fn ev_board(board: &[Column; 7]) -> i8{
    match ev_columns(board) {
        1 => return 1,
        -1 => return -1,
        _ => {
            match ev_rows(board) {
                1 => return 1,
                -1 => return -1,
                _ => return ev_diagonals(board)
            }       
        }
    }
}


fn get_move_value(board: &mut [Column; 7], for_us: bool, depth: u8) -> (u8, i8) {
    let mut results= [(0u8, 0); 7];
    for i in 0..7 {
        if board[i].len >= 6 {continue;}

        board[i].add_to_column(for_us);


        results[i] = (i as u8, ev_board(board));
        let a = ev_board(board);
        //print_board(board);
        //println!("{}", a);
        if a == 1 && for_us {
            board[i].remove_from_column();
            return (i as u8, 1);
        }
        if a == -1 && !for_us {
            board[i].remove_from_column();
            return (i as u8, -1);
        }

        if depth != 0 {
            let (_, x) = get_move_value(board, !for_us, depth-1);
            results[i] = (i as u8, x + x.signum());
        }

        board[i].remove_from_column();
    }

    if let Some(a) = results.iter().filter(|(i, v)| v.is_positive()).min_by_key(|(_, y)| y) {
        return *a;
    } else {


        if let Some(a) = results.iter().find(|(_, y)| y.eq(&0)){
            return *a;
        } else {
            if let Some(a) = results.iter().max_by_key(|(_, y)| y){
                return *a;
            } else {
                panic!("NO POSSIBLE MOVES. PROBABLY COULD HAVE MADE IT OPTIONAL BUT I CANT BE ASKED");
            }
        }
    }
}

fn solve(board: &mut [Column; 7]) -> u8 {
    return get_move_value(board, true, 5).0;
}

fn print_board(board: &[Column; 7]){
    let print_line = |i: u8| board.map(|x| if x.len <= i {" "} else {if x.our_tokens[usize::from(i)] {"O"} else {"X"}});
    println!("{:-<17}", "");
    for i in (0..6).rev() {
        println!("| {} |", print_line(i).join(" "));
    }
    println!("{:-<17}", "");
    println!("| 1 2 3 4 5 6 7 |")
}

fn string_to_board(s: &str) -> [Column; 7] {
    // O is us. X is them. Comma-separated columns. Bottom to top. Example:
    // OOOX,O,XX,,,OOOO,X
    let columns: Vec<&str> = s.split(',').collect();

    if columns.len() != 7 {panic!("Input must be 7 columns");}
    let mut board = [Column { len: 0, our_tokens: [false; 6] }; 7];

    for (i, s) in columns.iter().enumerate() {
        let mut ours = [false; 6];
        s.chars().enumerate().for_each(|(j, c)| ours[j] = c == 'O');
        board[i] = Column {
            len: s.len() as u8,
            our_tokens: ours,
        }; 
    }

    return board;
}


fn main() {
    
    let mut board: [Column; 7] = [Column {len: 0, our_tokens: [false; 6]}; 7];
    loop {
        print_board(&board);

        print!("Enter Column (1 - 7): ");
        let _ = stdout().flush();
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Bad input.");
        let c: u8 = s.trim().parse().ok().expect("Not a valid number");
        board[(c-1) as usize].add_to_column(true);
        print_board(&board);
        match ev_board(&board) {
            1 => {println!("You win"); break},
            -1 => {println!("You lose"); break},
            _ => {}
        }
        
        let them = solve(&mut board);
        board[them as usize].add_to_column(false);
        match ev_board(&board) {
            1 => {println!("You win"); print_board(&board); break},
            -1 => {println!("You lose"); print_board(&board); break},
            _ => {}
        }
    }
}

#[cfg(test)] // idk what most of this is doing but im cool so im writing tests
mod tests {
    use super::*;

    #[test]
    fn row_works() {
        let result = ev_rows(&string_to_board(",,,,,,"));
        assert_eq!(result, 0);

        let result2 = ev_rows(&string_to_board(",O,O,O,O,,"));
        assert_eq!(result2, 1);

        let result3 = ev_rows(&string_to_board(",,OX,XX,OX,XX,"));
        assert_eq!(result3, -1);
    }

    #[test]
fn columns_work() {
    let result = ev_columns(&string_to_board(",,,,,,"));
    assert_eq!(result, 0);

    let result2: i8 = ev_columns(&string_to_board("OOOO,,O,,O,,O"));
    assert_eq!(result2, 1);

    let result3 = ev_columns(&string_to_board("XXXX,,X,,X,,X"));
    assert_eq!(result3, -1);

    let result4 = ev_columns(&string_to_board("OXOXOX,,,,,,"));
    assert_eq!(result4, 0);
}

#[test]
fn diaganols_work() { // i made chat gpt write this for me. it got it wrong several times. and i ended up deleting most of them
    // Test with an empty board (no diagonals)
    // Board:
    //   ,,,,,,,
    // Representation: ",,,,,,,"
    let result = ev_diagonals(&string_to_board(",,,,,,"));
    assert_eq!(result, 0);

    // Test with a major diagonal (\) of 'O's, filler added below
    // Board:
    //   O,X,X,X,X,X,X
    //   X,O,X,X,X,X,X
    //   X,X,O,X,X,X,X
    //   X,X,X,O,X,X,X
    //   X,X,X,X,X,X,X
    //   X,X,X,X,X,X,X
    // Representation: "OXXXXX,XOXXXX,XXOXXX,XXXOXX,XXXXX,X,X"
    let result2 = ev_diagonals(&string_to_board("OXXXXX,XOXXXX,XXOXXX,XXXOXX,XXXXX,X,X"));
    assert_eq!(result2, 1);

    // Test with a minor diagonal (/) of 'X's, filler added below
    // Board:
    //   X,X,X,X,X,X,X
    //   X,X,X,X,X,X,X
    //   X,X,X,X,O,X,X
    //   X,X,X,O,X,X,X
    //   X,X,O,X,X,X,X
    //   X,O,X,X,X,X,X
    // Representation: "XXXXXX,XXXXXX,XXXXXO,XXXXOX,XXXOXX,XXOXXX,XOXXXX"
    let result3 = ev_diagonals(&string_to_board("XXXXXX,XXXXXX,XXXXXO,XXXXOX,XXXOXX,XXOXXX,XOXXXX"));
    assert_eq!(result3, -1);

    let result4 = ev_diagonals(&string_to_board("OXXOXX,XXXXXO,OXOXXX,XOXXXX,XXOXXX,XXXOXX,XXXXX"));
    assert_eq!(result4, -1);
}


#[test]
fn overall_ev_works(){
    let result3 = ev_board(&string_to_board("XXXXXX,XXXXXX,XXXXXO,XXXXOX,XXXOXX,XXOXXX,XOXXXX"));
    assert_eq!(result3, -1);
}

#[test]
fn one_move_problem(){
    let problem: &str = "O,O,O,,XXX,O,O";
    assert_eq!(get_move_value(&mut string_to_board(problem), true, 1).0, 3);
}


#[test]
fn three_move_problem(){
    let problem = "O,XOXO,X,X,O,XO,OX";
    
    assert_eq!(get_move_value(&mut string_to_board(problem), true, 3).0, 3);
}

#[test]
fn four_move_problem(){
    let problem = "O,O,X,XOXO,X,O,XXO";
    let mut a = &mut string_to_board(problem);
    assert_eq!(get_move_value(&mut a, true, 4).0, 1);
}

#[test]
fn aaaa(){
    let p = ",,O,OXOX,XOOX,XO,X";
    let mut a = &mut string_to_board(p);
    assert_eq!(get_move_value(&mut a, true, 3).0, 5);
}


}

