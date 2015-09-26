mod pentago;

fn main() {
    let g = pentago::game::Game::new(3, 3, 5);
    println!("{:?}", g.board.orient(1, &vec![true, true, false], &vec![1, 2, 0]));
}
