mod pentago;
extern crate num;

use self::num::traits::ToPrimitive;
use pentago::game_configuration::GameConfiguration;

fn main() {
    let gc = GameConfiguration::new(2, 3, 5);

    let gs = gc.init_state();

    println!("{:?}", gc.val(gs).to_u64());

}

