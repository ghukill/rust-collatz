
use std::env;
use terminal_size::{Width, Height, terminal_size};
use textplots::{Chart, Plot, Shape};


fn main() {

    // get args
    let args: Vec<String> = env::args().collect();

    // get input number
    let mut i: u32 = args[1].parse::<u32>().unwrap();
    let seed_value = i.clone();

    // bookkeeping
    let mut max_altitude: u32 = 1;
    let mut step_count: u32 = 1;
    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((step_count as f32, i as f32));

    // loop until i == 1 (NOTE: because a reference, this check works)
    while i > 1 {

        // println!("working on: {}...",i);

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

        // push to altitudes vector
        points.push((step_count as f32, i as f32));
    }

    println!("Seed value: {}, max altitude: {}, step count: {}.  Now buckle up for a pretty graph:", seed_value, max_altitude, step_count);

    // graph
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        Chart::new((w as f32 *1.5 ) as u32, (h as f32*1.5) as u32, 1 as f32, step_count as f32)
            .lineplot(&Shape::Lines(&points)).display();
    } else {
        println!("Unable to get terminal size, using some defaults...");
        Chart::new(200, 50, 1 as f32, step_count as f32)
            .lineplot(&Shape::Lines(&points)).display();
    }


}
