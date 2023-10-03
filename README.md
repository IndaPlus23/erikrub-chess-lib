Nytt Game = let mut game = Game::new();

pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>>{
    Ger möjliga moves att göra

    _position måste ha format på t.ex. "a1" / "h8"
}


pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
    Gör move om möjligt

    _from och _to måste ha format på t.ex. "a1" / "h8"
    
}


pub fn set_promotion(&mut self, _piece: &str) {
    När en bonde når kanten sätts GameState till SetPromotion

    Då måste man välja promotion innan man gör något annat.

    _piece ska ha formatet Queen = "q", Bishop = "b", Knight = "kn", Rook = "r"
}
