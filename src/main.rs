mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;


fn main() {
    let g = pentago::game::Game::new(2, 3, 5);

    let b = g.new_board();

    println!("{:?}", g.val(b).to_u64());

    // let b = g.board.orient(1, &vec![true, true, false], &vec![1, 2, 0]);

    // println!("{:?}", b.quadrants[1]);

}

