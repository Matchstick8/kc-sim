use std::env;
use rand::Rng;
use std::time::Duration;
use std::thread::sleep;

struct Vec2{
    x: f32,
    y: f32
}

struct Slice{
	freq: f32,
	pos: Vec2,
	mvec: Vec2,
	dist: f32,
	bond: bool
}

impl Slice {
	fn new(freq: f32, pos: Vec2, mvec: Vec2) -> Slice {
		Slice {
			freq: freq,
			pos: pos,
			mvec: mvec,
			dist: 0.0,
			bond: false
		}
	}
}

fn main() {
    let mut rng = rand::thread_rng();
	let args: Vec<String> = env::args().collect();

    // sim  conditions
	let sim_rate = 144; // "frames per second"
    let max_slices = 16; //currently an  arbtitrary number

    // global conditions
    let mut slices = Vec::new();

    // output settings
    let show_global = true;
    let mut show_slices = true;

	if args.len() > 1 {
		if args[1] == "slices" {
			if args[2] == "show" {
				show_slices = true;
			}
			if args[2] == "hide"{
				show_slices = false;
			}
		}
	}

	let mut sim_frame = 0;
    loop {
		sim_frame += 1;
		sleep(Duration::from_millis(1000/sim_rate));
		print!("\x1B[2J\x1B[1;1H"); // took 30 minutes to find this, clears the terminal.

    	// simulation logic
		let production_chance: f32 = rng.gen_range(0.0..100.0);
		
    	if production_chance < 1.0  && slices.len() < max_slices {
    		let freq = rng.gen::<f32>()*100.0;
    		let pos = Vec2{x: rng.gen::<f32>(), y: rng.gen::<f32>()};
    		let mvec = Vec2{x: rng.gen::<f32>(), y: rng.gen::<f32>()}; 
    		let slice = Slice::new(freq, pos, mvec);
    		slices.push(slice);
    	}

    	for n in 1..slices.len() {
    		slices[n-1].pos.x += slices[n-1].mvec.x;
    		slices[n-1].pos.y += slices[n-1].mvec.y;

			let direction_chance:f32 = rng.gen_range(0.0..100.0);
			// please improve, probably with a function but I couldn't be bothered
    		if direction_chance < 0.01 {
    			slices[n-1].mvec.x = rng.gen_range(-1.0..1.0);
    			slices[n-1].mvec.y = rng.gen_range(-1.0..1.0);
    		}

			for i in 1..slices.len() {
				if n-1 != i-1 {
					let x_dist = slices[i-1].pos.x - slices[n-1].pos.x;
					let y_dist = slices[i-1].pos.y - slices[n-1].pos.y;
					let new_dist = (x_dist*x_dist + y_dist*y_dist).sqrt();
					if new_dist < slices[n-1].dist {
						slices[n-1].dist = new_dist;
					}
				}
			}
    	}

		// terminal output
		println!("simframe {} ({} fps)", sim_frame, sim_rate);
		if show_global {
			let mut max_msg = "";
			if slices.len() == max_slices {
				max_msg = "(max)";
			}
	    	println!("slices: {} {}", slices.len(), max_msg);
		}
		if show_slices {
			println!("--- slices ---------------------------------------------------");
			for n in 1..slices.len() {
				// format values to list only one or two decimal places
				let freq = format!("{:.2}",  slices[n-1].freq);
				let posx = format!("{:.2}",  slices[n-1].pos.x);
				let posy = format!("{:.2}",  slices[n-1].pos.y);
				let mvecx = format!("{:.2}",  slices[n-1].mvec.x);
				let mvecy = format!("{:.2}",  slices[n-1].mvec.y);
				let dist = format!("{:.2}", slices[n-1].dist);
				println!("slice {:3}: freq: {:5} |  pos: ({:12}, {:12})  mvec: ({:5}, {:5}) | nearest slice: {:5}", n, freq, posx, posy, mvecx, mvecy, dist);
			}
		}
    }
}
