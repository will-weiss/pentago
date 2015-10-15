extern crate time;
mod pentago;

use pentago::configuration::Configuration;
use self::time::get_time;


fn main() {
    println!("STARTED AT: {:?}", get_time());

    // let initial_state = Configuration::new(2, 2, 3).init_state();

    let gc = Configuration::new(2, 2, 3);

    println!("{:?}", gc.all_lines);

    // let result = initial_state.full_result();

    // let states_calculated = initial_state.states_calculated();

    // println!("THE RESULT IS: {:?}", result);

    // println!("TOTAL STATES CALCULATED: {:?}", states_calculated);

    // println!("FINISHED AT: {:?}", get_time());

}
