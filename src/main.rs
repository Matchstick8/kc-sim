use rand::Rng;
struct Vec2{
    x: f32,
    y: f32
}

struct Slice{
	freq: f32,
	pos: Vec2,
	mvec: Vec2
}

impl Slice {
	fn new(freq: f32, pos: Vec2, mvec: Vec2) -> Slice {
		Slice {
			freq: freq,
			pos: pos,
			mvec: mvec
		}
	}

	fn get_freq (&self) -> f32 {
		self.freq
	}

	/*
	fn get_pos (&self) -> Tuple {
		(self.pos.x, self.pos.y)
	}
	*/
}

fn main() {
    let mut rng = rand::thread_rng();

    // sim  conditions
    let max_slices = 16; // currently an  arbtitrary number

    // global conditions
    let mut slices = Vec::new();

    // output settings
    let show_global = true;
    let show_slices = true;

	// make this into a loop with a consistent interval
    loop {
		print!("\x1B[2J\x1B[1;1H"); // took 30 minutes to find this, clears the terminal.

    	// simulation logic
		let production_chance: f32 = rng.gen_range(0.0..100.0);
		
    	if production_chance < 0.01 && slices.len() < max_slices {
    		let pos = Vec2{x: rng.gen::<f32>(), y: rng.gen::<f32>()};
    		let mvec = Vec2{x: rng.gen::<f32>(), y: rng.gen::<f32>()}; 
    		let slice = Slice::new(rng.gen::<f32>(), pos, mvec);
    		slices.push(slice);
    	}

    	for n in 1..slices.len() {
    		slices[n-1].pos.x += slices[n-1].mvec.x;
    		slices[n-1].pos.y += slices[n-1].mvec.y;

			let direction_chance:f32 = rng.gen_range(0.0..100.0);
    		if direction_chance < 0.005 {
    			slices[n-1].mvec.x = rng.gen::<f32>();
    			let negchance_x:i8 = rng.gen_range(0..100);
    			if negchance_x  <= 50 {
    				slices[n-1].mvec.x *= -1.0;
    			}
    			slices[n-1].mvec.y = rng.gen::<f32>();
    			let negchance_y:i8 = rng.gen_range(0..100);
    			if negchance_y <= 50 {
    				slices[n-1].mvec.y *= -1.0;
    			}
    		}
    	}

		// terminal output
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
				println!("slice {:3}: freq: {:12} |  pos: ({:12}, {:12}), mvec: ({:12}, {:12})", n, slices[n-1].freq, slices[n-1].pos.x, slices[n-1].pos.y, slices[n-1].mvec.x, slices[n-1].mvec.y);
			}
		}
    }
}
