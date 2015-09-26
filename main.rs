mod pentago;

fn main() {
    let g = pentago::game::Game::new(2, 3, 5);
    println!("{:?}", g);
}
