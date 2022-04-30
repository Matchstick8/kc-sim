use rand::Rng;

struct Vec2{
    x: f32,
    y: f32
}

struct Spline {
    aur: f32,
    aus: f32,
    aur_sow: bool,
    aus_sow: bool,
    pos: Vec2
}

impl Spline {
    fn new(aur: f32, aus: f32, aur_sow: bool, aus_sow: bool, pos: Vec2) -> Spline{
        Spline {
            aur: aur,
            aus: aus,
            aur_sow: aur_sow,
            aus_sow: aus_sow,
            pos: pos
        }
    }

    fn get_avg(&self) -> f32 {
        (self.aur + self.aus)/2.0
    }

    fn get_status(&self) -> String{
        if self.aur_sow{
            "sewn at aurora".to_string()
        }
        else if self.aus_sow{
            "sewn at australis".to_string()
        }
        else{
            "unsewn".to_string()
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // starting conditions
    let starting_splines = 16;

    for _n in 1..starting_splines {
        let pos = Vec2{x: rng.gen::<f32>(), y: rng.gen::<f32>()};
        let s = Spline::new(rng.gen::<f32>(), rng.gen::<f32>(), false, false, pos);
        println!("spline (x: {}, y: {}): {} w/ avg of {}, aur: {}, aus: {}", s.pos.x, s.pos.y ,s.get_status(), s.get_avg(), s.aur, s.aus)
    }

}
