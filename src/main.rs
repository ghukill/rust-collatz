
use std::env;



fn main() {

    // get args
    let args: Vec<String> = env::args().collect();

    // get input number
    let mut i: u32 = args[1].parse::<u32>().unwrap();

    println!("Your input number is {}, now let's determine the Collatz conjecture altitude + steps", i);

    // bookkeeping
    let mut max_altitude: u32 = 1;
    let mut step_count: u32 = 0;

    // loop until i == 1 (NOTE: because a reference, this check works)
    while i > 1 {

        println!("working on: {}...",i);

        // bump step count
        step_count += 1;

        // tripe/halve
        if i % 2 == 0  {
            i = i / 2;
        }
        else {
            i = (3 * i) + 1;
        }

        // bump max altitude
        if i > max_altitude {
            max_altitude = i;
        }
    }

    println!("Finis!  Max altitude: {}, step count: {}.", max_altitude, step_count)

}
