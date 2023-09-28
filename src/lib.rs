use std::fmt;
use std::collections::HashMap;
use std::cmp;
use std::str;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

pub struct Game {
    state: GameState,
    turn: PieceColor,
    gameboard: Vec<Option<Piece>>,
    possible_moves: HashMap<i16, Vec<i16>>,
    distances: HashMap<i16, Vec<i16>>,
    string_to_int: HashMap<String, i16>,
    int_to_string: HashMap<i16, String>,
    movements: Vec<u16>,
    direction_finder: HashMap<u16, usize>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
            turn: PieceColor::White,
            movements: vec![-8, -7, 1, 9, 8, 7, -1, -9],
            direction_finder: HashMap::from([(7,5),(8,4),(9,3),(-7,1),(-8,0),(-9,7),]),
            distances: Self::generate_distances(),
            string_to_int: Self::string_to_int(),
            int_to_string: Self::int_to_string(&self),
            gameboard: Self::new_board(),
            possible_moves: Self::get_all_possible_moves(&self),
        }
    }

    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        let pos = &self.string_to_int.get(_postion);
        let piece: &Option<Piece> = &self.gameboard.get(pos);
        match piece {
            Some(piece) => {
                if &self.turn == piece.piececolor {
                    let mut moves: Vec<String> = Vec::new();
                    for _move in &self.possible_moves.get(pos).iter() {
                        moves.push(&self.int_to_string[_move]);
                        return Some(moves);
                    }
                }else {
                    None
                }
            },
            None => None,
        }
    }
    
    fn get_all_possible_moves(&self) -> HashMap<i16, Vec<int>> {
        let mut map: HashMap<i16, Vec<int>> = HashMap::new();

        let mut position = 0;
        while position < 64 {

            let piece: &Option<Piece> = &self.gameboard.get(position);
            match piece {
                Some(piece) => {
                    if piece.piececolor == &self.turn {
                        match piece.piecetype{
                            PieceType::King => {map.insert(position, self.possible_moves(position, 0, 1, &self.turn, true))},
                            PieceType::Queen => {map.insert(position, self.possible_moves(position, 0, 1, &self.turn, false))},
                            PieceType::Bishop => {map.insert(position, self.possible_moves(position, 1, 2, &self.turn, false))},
                            PieceType::Knight => {map.insert(position, self.possible_moves_knight(position, &self.turn))},
                            PieceType::Rook => {map.insert(position, self.possible_moves(position, 0, 2, &self.turn, true))},
                            PieceType::Pawn => {map.insert(position, self.possible_moves_pawn(position, &self.turn, piece.hasmoved))},
                        }
                    }
                },
                None => None,
            }

            position = position + 1;
        }
        return map;
    }

    fn possible_moves(&self, position: usize, start: usize, add: usize, turn: PieceColor, king: bool) -> Vec<i16> {
        let mut moves: Vec<i16> = Vec::new();
        let mut state: GameState = GameState::InProgress;

        let mut direction = start;

        while direction < 8 {
            let mut pos = position;
            let mut range = 0;
            while range < &self.distances.get(&position)[&direction] {
                newpos = newpos + &self.movements[direction];

                let piece: &Option<Piece> = &self.gameboard.get(newpos);

                match piece {
                    Some(piece) => {
                        if turn == piece.piececolor {
                            break;
                        }
                        moves.push(newpos);
                        if turn != piece.piececolor {
                            break;
                        }
                        if piece.piecetype == PieceType::King {
                            break;
                        }
                    },
                    None => None,
                }
                range = range + 1;
            }


            direction = direction + add;
        }
    }

    fn possible_moves_knight(&self, position: usize, turn: PieceColor) -> Vec<i16> {
        let mut moves: Vec<i16> = Vec::new();
        let mut first_direction = 0;
        while first_direction < 8 {
            if &self.distances.get(&position)[&first_direction] >= 2 {
                let mut first_pos: usize = position + &self.movements[&first_direction] * 2;
                let mut start: usize = 0;
                let mut end: usize = 4;
                if (((first_direction + 2) / 2) + 1) % 2 == 0 {
                    start = start + 2;
                    end = end + 2;
                }
                let mut second_direction: usize = start;
                while second_direction <= end {
                    if &self.distances.get(&first_pos)[&second_direction] >= 1 {
                        let mut second_pos: usize = first_pos + &self.movements.get(&second_direction);
                        let piece: &Option<Piece> = &self.gameboard.get(second_pos);
                        match piece {
                            Some(piece) => {
                                if turn != piece.piececolor {
                                    moves.push(second_pos);
                                }
                            },
                            None => None,
                        }
                    }

                    second_direction = second_direction + 4;
                }
            }
            
            first_direction = first_direction + 2;
        }
        moves
    }

    fn possible_moves_pawn(&self, position: usize, turn: PieceColor, hasmoved: bool) -> Vec<i16> {
        let mut moves: Vec<i16> = Vec::new();

        let mut reverse = -1;
        if turn == PieceColor::Black {
            reverse = reverse + 2;
        }

        let mut direction = 7;
        while direction <= 9 {
            if &self.distances.get(&position)[&self.direction_finder.get(&(direction*reverse))] > 0 {
                if direction == 8 {
                    if hasmoved {
                        
                        let mut newpos: usize = position + (direction * reverse);
                        let piece: &Option<Piece> = &self.gameboard.get(&newpos);
                        match piece {
                            Some(piece) => (),
                            None => {
                                moves.push(newpos);
                            },
                        }
                    } else {
                        let mut newpos: usize = position;
                        let mut range: usize = 0;
                        while range < 2 {
                            newpos = newpos + (direction * reverse);
                            let piece: &Option<Piece> = &self.gameboard.get(&newpos);
                            match piece {
                                Some(piece) => (),
                                None => {
                                    moves.push(newpos);
                                },
                            }
                            range = range + 1;
                        }
                    }
                } else {
                    let mut newpos: usize = position + (direction * reverse);
                    let piece:&Option<Piece> = &self.gameboard.get(&newpos);
                    match piece {
                        Some(piece) => {
                            if piece.piececolor != turn {
                                moves.push(newpos);
                            }
                        },
                        None => None,
                    }
                }
            }
            direction = direction + 1;
        }
        moves
    }

    fn string_to_int() -> HashMap<String, i16> {
        let mut map: HashMap<String, i16> = HashMap::new();

        let mut count: i16 = 0;

        let mut row: u8 = 8;
        while row > 0 {
            let mut col: u8 = 0;
            while col < 8 {
                let mut s = String::new();
                s.push((col + 97) as char);
                s.push((row + 48) as char);

                map.insert(s, count);

                count = count + 1;
                col = col + 1;
            }
            row = row - 1;
        }
        map
    }

    fn int_to_string(&self) -> HashMap<i16, String> {
        let mut map: HashMap<i16, String> = HashMap::new();

        for (key, val) in &self.string_to_int.iter() {
            map.insert(val, key);
        }
        map
    }

    fn generate_distances() -> HashMap<i16, Vec<i16>> {
        let mut distances: HashMap<i16, Vec<i16>> = HashMap::new();
        let mut count: i16 = 0;

        let mut row: i16 = 0;
        while row < 8 {
            let mut col: i16 = 0;
            while col < 8 {

                let up = row;
                let right = 7 - col;
                let down = 7 - row;
                let left = col;

                let up_right = cmp::min(up, right);
                let down_right = cmp::min(down, right);
                let down_left = cmp::min(down, left);
                let up_left = cmp::min(up, left);

                let v: Vec<i16> = vec![up, up_right, right, down_right, down, down_left, left, up_left];
                distances.insert(count, v);

                count = count + 1;
                col = col + 1;
            }
            row = row + 1;
        }
        distances
    }
    
    fn new_board() -> Vec<Option<Piece>> {
        let mut board: Vec<Option<Piece>> = Vec::new();
        add_black_backrow(&mut board);
        add_black_pawns(&mut board);
        add_empty_rows(&mut board);
        add_white_pawns(&mut board);
        add_white_backrow(&mut board);

        return board;

        fn add_black_backrow(board: &mut Vec<Option<Piece>>){
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Knight, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::King, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Queen, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::King, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::Black)));
        }
        fn add_black_pawns(board: &mut Vec<Option<Piece>>){
            for _x in 0..8{
                board.push(Some(Piece::new(PieceType::Pawn, PieceColor::Black)));
            }
        }
        fn add_empty_rows(board: &mut Vec<Option<Piece>>){
            for _x in 0..32 {
                board.push(None);
            }
        }
        fn add_white_pawns(board: &mut Vec<Option<Piece>>){
            for _x in 0..8{
                board.push(Some(Piece::new(PieceType::Pawn, PieceColor::White)));
            }
        }
        fn add_white_backrow(board: &mut Vec<Option<Piece>>){
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Knight, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::King, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Queen, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Knight, PieceColor::White)));
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::White)));
        }
    }
}

pub struct Piece {
    piecetype: PieceType,
    piececolor: PieceColor,
    hasmoved: bool,
}

impl Piece {
    pub fn new(piecetype: PieceType, piececolor: PieceColor) -> Piece {
        Piece{
            piecetype,
            piececolor,
            hasmoved: false
        }
    }

    pub fn get_piecetype(&self) -> PieceType {
        &self.piecetype
    }

    pub fn get_piececolor(&self) -> PieceColor {
        &self.piececolor
    }
}

pub enum PieceType {
	King,
	Queen,
	Bishop,
	Knight,
	Rook,
	Pawn,
}

pub enum PieceColor {
    White,
    Black,
}


/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}