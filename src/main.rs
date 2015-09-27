mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::game_configuration::GameConfiguration;

fn main() {
    let gc = GameConfiguration::new(3, 4, 5);
    println!("{:?}", gc.square_coords);

    // let gs = gc.init_state();


}

