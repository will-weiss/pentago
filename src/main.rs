mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::game_state::GameState;
use pentago::game_configuration::GameConfiguration;
use pentago::color::Color::{Black, White};


fn val_of(gs: &GameState) {
    println!("{:?}", gs.val().to_u64().unwrap());
}

fn main() {
    let gc = GameConfiguration::new(2, 3, 5);

    println!("{:?}", gc);

    let s0 = gc.init_state();
    val_of(&s0);
    let s1 = s0.place(0, 0, White);
    val_of(&s1);


    let mut s = s1.rotate(0, 1);
    val_of(&s);

    s = s.rotate(0, 1);
    val_of(&s);

    s = s.rotate(0, 1);
    val_of(&s);

    s = s.rotate(0, 1);
    val_of(&s);







    // let gs = gc.init_state();

    // println!("{:?}", gs.val().to_u64());


}

