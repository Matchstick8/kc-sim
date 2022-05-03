use rand::Rng;

struct Vec2{
    x: f32,
    y: f32
}

struct Slice{
	freq: f32,
	pos: Vec2
}

impl Slice {
	fn new(freq: f32, pos: Vec2) -> Slice {
		Slice {
			freq: freq,
			pos: pos
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
    let max_slices = 16;

    // global conditions
    let mut slices = 0;

    // output settings
    let show_global = true;
    let show_slices = true;


    loop {
		print!("\x1B[2J\x1B[1;1H"); // took 30 minutes to find this, clears the terminal.
    
		let production_chance: f32 = rng.gen_range(0.0..100.0);
    	if production_chance < 0.01 {
    		slices += 1;
    	}

		// terminal output
		if show_global {
	    	println!("slices: {}", slices);
		}
		if show_slices {
			println!("--- slices ---------------------------------------------------");
			for n in 1..slices {
				println!("slice {}", n);
			}
		}
    }

}

