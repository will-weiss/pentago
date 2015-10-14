mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::state::State;
use pentago::configuration::Configuration;
use pentago::board::Color::{Black, White};


fn val_of(gs: &State) {
    println!("{:?}", gs.val().to_u64().unwrap());
}

fn main() {
    let gc = Configuration::new(2, 3, 5);

    println!("{:?}", gc);

    let s0 = gc.init_state();
    val_of(&s0);


    let s1 = s0.place(0, 0, White);

    let s1 = s0.place(0, 2, Black);
    val_of(&s1);


    let mut s = s1.rotate_single_quadrant(0, 1);

    val_of(&s);

    s = s.rotate_single_quadrant(0, 1);
    val_of(&s);

    s = s.rotate_single_quadrant(0, 1);
    val_of(&s);

    s = s.rotate_board(1);
    val_of(&s);

    s = s.rotate_board(1);
    val_of(&s);

    s = s.rotate_board(1);
    val_of(&s);

    s = s.rotate_board(1);
    val_of(&s);







    // let gs = gc.init_state();

    // println!("{:?}", gs.val().to_u64());


}

