use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    SetPromotion,
    Check,
    GameOver,
}

#[derive(Clone)]
pub struct Game {
    state: GameState,
    turn: PieceColor,
    gameboard: Vec<Option<Piece>>,
    possible_moves: HashMap<usize, Vec<usize>>,
    distances: HashMap<usize, Vec<i16>>,
    string_to_int: HashMap<String, usize>,
    int_to_string: HashMap<usize, String>,
    movements: Vec<i16>,
    direction_finder: HashMap<i16, i16>,
    promotion_pos: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            state: GameState::InProgress,
            turn: PieceColor::White,
            movements: vec![-8, -7, 1, 9, 8, 7, -1, -9],
            direction_finder: HashMap::from([(7, 5), (8, 4), (9, 3), (-7, 1), (-8, 0), (-9, 7)]),
            distances: Self::generate_distances(),
            string_to_int: Self::string_to_int(),
            int_to_string: HashMap::new(),
            gameboard: Self::new_board(),
            possible_moves: HashMap::new(),
            promotion_pos: None,
        };

        game.firstload();

        game
    }

    fn firstload(&mut self) {
        self.possible_moves = self.get_all_possible_moves(self.turn).0;
        self.int_to_string = self.int_to_string();
    }

    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let pos = *self.string_to_int.get(_from).unwrap();
        let newpos = *self.string_to_int.get(_to).unwrap();

        let allowed: bool;
        let is_pawn: bool;
        {
            let board = &mut self.gameboard;
            let piece = board.get_mut(pos).unwrap();

            allowed = match piece {
                Some(piece) if piece.piececolor == self.turn => self
                    .possible_moves
                    .get(&pos)
                    .unwrap()
                    .iter()
                    .any(|_move| *_move == newpos),
                _ => false,
            };

            is_pawn = match piece {
                Some(piece) if piece.piecetype == PieceType::Pawn => {
                    piece.hasmoved = true;
                    true
                }
                _ => false,
            };

            if allowed {
                board.swap(pos, newpos);
                board[pos] = None;
            }
        }

        let mut state = GameState::InProgress;
        if allowed {
            if is_pawn {
                state = self.check_promotion(newpos);
            }

            if state == GameState::SetPromotion {
                self.promotion_pos = Some(newpos);
                self.possible_moves = HashMap::new();
            }
            else {
                state = self.get_all_possible_moves(self.turn).1;

                if self.turn == PieceColor::White {
                    self.turn = PieceColor::Black;
                } else {
                    self.turn = PieceColor::White;
                }

                self.possible_moves = self.get_all_possible_moves(self.turn).0;

            }

            

            

            
            
        }




        

        self.state = state;
        println!("{:?}", self.state);

        return Some(state);
    }

    pub fn set_promotion(&mut self, _piece: &str) { //Queen = "q", Bishop = "b", Knight = "kn", Rook = "r"
    
        let mut allowed = false;
        {
            if self.promotion_pos != None {
                allowed = true;
            }
        }

        if allowed {
            let typee = self.string_to_piece(_piece);
            let board = &mut self.gameboard;
            let mut piece = board.get_mut(self.promotion_pos.unwrap()).unwrap();
            match piece {
                Some(piece) => {
                    piece.piecetype = typee;
                },
                None => {},
            }
            self.promotion_pos = None;

            let mut turn = self.turn;
            let mut state = self.get_all_possible_moves(turn).1;

            if self.turn == PieceColor::White{
                
                self.turn = PieceColor::Black;
            } else {
                self.turn = PieceColor::White;
            }

            println!("---------PROM");

            self.possible_moves = self.get_all_possible_moves(self.turn).0;

            self.state = state;
        }
        
    }

    fn string_to_piece(&self, _piece: &str) -> PieceType {
        let mut typee = PieceType::Pawn;
        if _piece == "q" {
            typee = PieceType::Queen;
        } else if _piece == "b" {
            typee = PieceType::Bishop;
        } else if _piece == "kn" {
            typee = PieceType::Knight;
        } else if _piece == "r" {
            typee = PieceType::Rook;
        }
        return typee;
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    fn check_game_state(&self) -> GameState {
        GameState::InProgress
    }














    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        let pos = *self.string_to_int.get(_postion).unwrap();
        let piece: &Option<Piece> = self.gameboard.get(pos).unwrap();
        match piece {
            Some(piece) => {
                if self.turn == piece.piececolor {
                    let mut moves: Vec<String> = Vec::new();
                    for _move in self.possible_moves.get(&pos).unwrap().iter() {
                        moves.push(self.int_to_string.get(_move).unwrap().to_owned());
                    }
                    return Some(moves);
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn get_all_possible_moves(&self, turn: PieceColor) -> (HashMap<usize, Vec<usize>>, GameState) {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut allowed_direction: HashMap<usize, usize> = HashMap::new(); //Blocking and axis
        let mut state = GameState::InProgress;

        let mut posi = 0;
        while posi < 64 {//Is piece blocking check
            let piece: &Option<Piece> = self.gameboard.get(posi).unwrap();
            match piece {
                Some(piece) => {
                    if piece.piececolor != turn {
                        let mut respons: Option<(usize, usize)> = None;
                        match piece.piecetype {
                            PieceType::King => (),
                            PieceType::Queen => {
                                respons = self.blocking_check(posi, 0, 1, turn);
                            }
                            PieceType::Bishop => {
                                respons = self.blocking_check(posi, 1, 2, turn);
                            }
                            PieceType::Knight => (),
                            PieceType::Rook => {
                                respons = self.blocking_check(posi, 0, 2, turn);
                            }
                            PieceType::Pawn => (),
                        }
                        match respons {
                            Some(respons) => {
                                allowed_direction.insert(respons.0, respons.1);
                            }
                            None => (),
                        }
                    }
                }
                None => (),
            }
            posi = posi + 1;
        }

        let mut position = 0;
        while position < 64 {
            let piece: &Option<Piece> = self.gameboard.get(position).unwrap();
            match piece {
                Some(piece) => {
                    if piece.piececolor == self.turn {
                        let allowed = allowed_direction.get(&position);

                        let mut response: (Vec<usize>, GameState) = (Vec::new(), GameState::InProgress);
                    
                        match allowed {
                            Some(allowed) => match piece.piecetype {
                                PieceType::King => (),
                                PieceType::Queen => {
                                    response = self.possible_moves(position, 0, 1, turn, false, *allowed,);
                                }
                                PieceType::Bishop => {
                                    response = self.possible_moves(position, 1, 2, turn, false, *allowed,);
                                }
                                PieceType::Knight => (),
                                PieceType::Rook => {
                                    response = self.possible_moves(position, 0, 2, turn, true, *allowed,);
                                }
                                PieceType::Pawn => {
                                    response = self.possible_moves_pawn(position, turn, piece.hasmoved, *allowed,);
                                }
                            },
                            None => match piece.piecetype {
                                PieceType::King => {
                                    response = self.possible_moves(position, 0, 1, turn, true, 100);
                                }
                                PieceType::Queen => {
                                    response = self.possible_moves(position, 0, 1, turn, false, 100,);
                                }
                                PieceType::Bishop => {
                                    response = self.possible_moves(position, 1, 2, turn, false, 100,);
                                }
                                PieceType::Knight => {
                                    response = self.possible_moves_knight(position, turn);
                                }
                                PieceType::Rook => {
                                    response = self.possible_moves(position, 0, 2, turn, true, 100,);
                                }
                                PieceType::Pawn => {
                                    response = self.possible_moves_pawn(position, turn, piece.hasmoved, 100,);
                                }
                            },
                        }
                        map.insert(position, response.0);
                        state = response.1;
                    }
                }
                None => (),
            }

            position = position + 1;
        }
        return (map, state);
    }

    fn possible_moves(
        &self,
        position: usize,
        start: usize,
        add: usize,
        turn: PieceColor,
        king: bool,
        allowed: usize,
    ) -> (Vec<usize>, GameState) {
        let mut moves: Vec<usize> = Vec::new();
        let mut state: GameState = GameState::InProgress;
       // println!("pos: {}", position);
        let mut direction: usize = start;

        while direction < 8 {
            if direction == allowed
                || direction + 4 == allowed
                || direction == allowed + 4
                || allowed == 100
            {
                let mut newpos = position;
                let mut range = 0;
                while range < self.distances.get(&position).unwrap()[direction as usize] {
                    newpos = (newpos as i16 + self.movements[direction as usize]) as usize;

                    let piece: &Option<Piece> = self.gameboard.get(newpos).unwrap();

                    match piece {
                        Some(piece) => {
                            if turn == piece.piececolor {
                                break;
                            }
                            moves.push(newpos);
                            if turn != piece.piececolor {
                                if piece.piecetype == PieceType::King {
                                    println!("Check {}", newpos);
                                    state = GameState::Check;
                                }
                                break;
                            }
                        }
                        None => {
                            moves.push(newpos);
                        }
                    }
                    if king {
                        // king can only move 1
                        break;
                    }
                    range = range + 1;
                }
            }

            direction = direction + add;
        }
        (moves, state)
    }

    fn possible_moves_knight(&self, position: usize, turn: PieceColor) -> (Vec<usize>, GameState) {
        let mut moves: Vec<usize> = Vec::new();
        let mut state = GameState::InProgress;

        let mut first_direction: usize = 0;
        while first_direction < 8 {
            if self.distances.get(&position).unwrap()[first_direction] >= 2 {
                let first_pos: usize =
                    (position as i16 + (self.movements[first_direction] * 2)) as usize;
                let mut start: usize = 0;
                let mut end: usize = 4;
                if (((first_direction + 2) / 2) + 1) % 2 == 0 {
                    start = start + 2;
                    end = end + 2;
                }

                let mut second_direction: usize = start;
                while second_direction <= end {
                    if self.distances.get(&first_pos).unwrap()[second_direction] >= 1 {
                        let test = *self.movements.get(second_direction).unwrap();

                        let second_pos: usize = (first_pos as i16 + test) as usize;
                        let piece: &Option<Piece> = self.gameboard.get(second_pos).unwrap();
                        match piece {
                            Some(piece) => {
                                if turn != piece.piececolor {
                                    moves.push(second_pos);
                                    if piece.piecetype == PieceType::King {
                                        state = GameState::Check;
                                    }
                                }
                            }
                            None => {
                                moves.push(second_pos);
                            }
                        }
                    }

                    second_direction = second_direction + 4;
                }
            }

            first_direction = first_direction + 2;
        }
        (moves, state)
    }

    fn check_promotion(&self, position: usize) -> GameState {
        if self.turn == PieceColor::White {
            let dist = self.distances.get(&position).unwrap()[0];
            if dist == 0 {
                return GameState::SetPromotion;
            } else {
                return GameState::InProgress;
            }
        } else {
            let dist = self.distances.get(&position).unwrap()[4];
            if dist == 0 {
                return GameState::SetPromotion;
            } else {
                return GameState::InProgress;
            }
        }
    }

    fn possible_moves_pawn(
        &self,
        position: usize,
        turn: PieceColor,
        hasmoved: bool,
        allowed: usize,
    ) -> (Vec<usize>, GameState) {
        let mut moves: Vec<usize> = Vec::new();
        let mut state = GameState::InProgress;

        println!("");
        println!("pos {}", position);
        println!("turn {:?}", turn);
        println!("aturn {:?}", self.turn);
        println!("");

        let mut reverse: i16 = -1;
        if turn == PieceColor::Black {
            reverse = reverse + 2;
        }

        println!("reverse {}", reverse);

        let mut direction: i16 = 7;
        while direction <= 9 {
            let dir = *self.direction_finder.get(&direction).unwrap(); //direction_finder: HashMap::from([(7, 5), (8, 4), (9, 3), (-7, 1), (-8, 0), (-9, 7)])
            if dir == allowed as i16
                || dir + 4 == allowed as i16
                || dir == (allowed as i16) + 4
                || allowed == 100
            {
                let distances = self.distances.get(&position).unwrap();
                let dir = *self.direction_finder.get(&(direction * reverse)).unwrap() as usize;
                let distance = distances[dir];

                if distance > 0 {
                    if direction == 8 {
                        if hasmoved {
                            let newpos: usize = (position as i16 + direction * reverse) as usize;
                            let piece: &Option<Piece> = self.gameboard.get(newpos).unwrap();
                            match piece {
                                Some(_piece) => (),
                                None => {
                                    moves.push(newpos);
                                }
                            }
                        } else {
                            let mut newpos: usize = position;
                            let mut range: usize = 0;
                            while range < 2 {
                                println!("old {}", newpos);
                                println!("dir {}", direction);
                                println!("rev {}", reverse);
                                newpos = (newpos as i16 + (direction * reverse)) as usize;
                                println!("new {}", newpos);
                                let piece  = self.gameboard.get(newpos).unwrap();
                                match piece {
                                    Some(_piece) => (),
                                    None => {
                                        moves.push(newpos);
                                    }
                                }
                                range = range + 1;
                            }
                        }
                    } else {
                        let newpos: usize = (position as i16 + direction * reverse) as usize;
                        let piece: &Option<Piece> = self.gameboard.get(newpos).unwrap();
                        match piece {
                            Some(piece) => {
                                if piece.piececolor != turn {
                                    moves.push(newpos);
                                    if piece.piecetype == PieceType::King {
                                        state = GameState::Check;
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
                direction = direction + 1;
            }
        }
        (moves, state)
    }

    fn blocking_check(
        &self,
        position: usize,
        start: usize,
        add: usize,
        turn: PieceColor,
    ) -> Option<(usize, usize)> {
        let mut direction: usize = start;
        while direction < 8 {
            let mut newpos: usize = position;
            let mut blocking: Option<usize> = None;
            let mut range: usize = 0;

            let _x: i16 = position as i16;
            let stop: usize = self.distances.get(&position).unwrap()[direction] as usize;

            while range < stop {
                newpos = ((newpos as i16) + self.movements[direction]) as usize;
                let piece: &Option<Piece> = self.gameboard.get(newpos).unwrap();

                match piece {
                    Some(piece) => {
                        if piece.piececolor != turn {
                            break;
                        }

                        if piece.piecetype != PieceType::King {
                            match blocking {
                                Some(_blocking) => break,
                                None => {
                                    blocking = Some(newpos);
                                }
                            }
                        } else {
                            match blocking {
                                Some(_blocking) => return Some((_blocking, direction)),
                                None => {}
                            }
                        }
                    }
                    None => {}
                }

                range = range + 1;
            }

            direction = direction + add;
        }
        return None;
    }

    fn blocking_check_pawn(&self, position: usize, turn: PieceColor) {
        if turn == PieceColor::Black { // Vit bonde
        } else { // Svart bonde
        }
    }

    fn string_to_int() -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();

        let mut count: usize = 0;

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

    fn int_to_string(&self) -> HashMap<usize, String> {
        let mut map: HashMap<usize, String> = HashMap::new();

        for (key, val) in self.string_to_int.iter() {
            map.insert(*val, (*key.clone()).to_string());
        }
        map
    }

    fn generate_distances() -> HashMap<usize, Vec<i16>> {
        let mut distances: HashMap<usize, Vec<i16>> = HashMap::new();
        let mut count: usize = 0;

        let mut row: usize = 0;
        while row < 8 {
            let mut col: usize = 0;
            while col < 8 {
                let up: i16 = row as i16;
                let right: i16 = 7 - col as i16;
                let down: i16 = 7 - row as i16;
                let left: i16 = col as i16;

                let up_right: i16 = cmp::min(up, right);
                let down_right: i16 = cmp::min(down, right);
                let down_left: i16 = cmp::min(down, left);
                let up_left: i16 = cmp::min(up, left);

                let v: Vec<i16> = vec![
                    up, up_right, right, down_right, down, down_left, left, up_left,
                ];
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

        fn add_black_backrow(board: &mut Vec<Option<Piece>>) {
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Knight, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::King, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Queen, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Bishop, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Knight, PieceColor::Black)));
            board.push(Some(Piece::new(PieceType::Rook, PieceColor::Black)));
        }
        fn add_black_pawns(board: &mut Vec<Option<Piece>>) {
            for _x in 0..8 {
                board.push(Some(Piece::new(PieceType::Pawn, PieceColor::Black)));
            }
        }
        fn add_empty_rows(board: &mut Vec<Option<Piece>>) {
            for _x in 0..32 {
                board.push(None);
            }
        }
        fn add_white_pawns(board: &mut Vec<Option<Piece>>) {
            for _x in 0..8 {
                board.push(Some(Piece::new(PieceType::Pawn, PieceColor::White)));
            }
        }
        fn add_white_backrow(board: &mut Vec<Option<Piece>>) {
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

#[derive(Clone)]
pub struct Piece {
    piecetype: PieceType,
    piececolor: PieceColor,
    hasmoved: bool,
}

impl Piece {
    pub fn new(piecetype: PieceType, piececolor: PieceColor) -> Piece {
        Piece {
            piecetype,
            piececolor,
            hasmoved: false,
        }
    }

    pub fn get_piecetype(&self) -> PieceType {
        self.piecetype
    }

    pub fn get_piececolor(&self) -> PieceColor {
        self.piececolor
    }


}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
        let mut board = String::new();
        board.push_str("\n");
        board.push_str("|:----------------------:");
        let mut count = 0;
        for piece in self.gameboard.iter() {
            if count % 8 == 0 {
                board.push_str("|");
                board.push_str("\n");
                board.push_str("|");
            }
            match piece {
                Some(piece) => {
                    let symbol: &str = match piece.piecetype {
                        PieceType::King => " K ",
                        PieceType::Queen => " Q ",
                        PieceType::Bishop => " B ",
                        PieceType::Knight => " Kn",
                        PieceType::Rook => " R ",
                        PieceType::Pawn => " P ",
                    };
                    board.push_str(symbol);
                }
                None => board.push_str(" * "),
            }

            count = count + 1;
        }
        board.push_str("|");
        board.push_str("\n");
        board.push_str("|:----------------------:|");
        /* build board representation string */

        write!(f, "{}", board)
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
        let mut game = Game::new();

        game.make_move("a2", "a4");
        game.make_move("b7", "b5");
        game.make_move("a4", "b5");
        game.make_move("b8", "a6");
        game.make_move("b5", "b6");
        game.make_move("d7", "d6");
        println!("{:?}", game);
        game.make_move("b6", "b7");
        println!("{:?}", game);
        game.make_move("c8", "d7");

        println!("{:?}", game);
        println!("-----------------------");
        //println!("{:?}", game.possible_moves);

        game.make_move("b7", "b8");
        println!("{:?}", game);

        println!("-----------------------");
        //println!("{:?}", game.possible_moves);
        game.set_promotion("q");

        println!("-----------------------");
        //println!("{:?}", game.possible_moves);

        println!("{:?}", game);

        println!("{:?}", game.get_game_state());

        //assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
