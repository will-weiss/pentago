mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::game_configuration::GameConfiguration;

fn main() {
    let gc = GameConfiguration::new(2, 3, 5);
    // println!("{:?}", gc.square_cfgs);

    let gs = gc.init_state();

    println!("{:?}", gs.val().to_u64());


}

