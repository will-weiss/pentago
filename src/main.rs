mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::game_configuration::GameConfiguration;
use pentago::color::Color::{Black, White};

fn main() {
    let gc = GameConfiguration::new(2, 3, 5);
    let gs = gc.init_state();


    println!("{:?}", gs.board.quadrants[0].place(0, &White).rotate(0));



    // let gs = gc.init_state();

    // println!("{:?}", gs.val().to_u64());


}

