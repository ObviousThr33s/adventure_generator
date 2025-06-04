use rand::{self, Rng};


pub fn gen_tv_static(num_frames:i32) -> Vec<Vec<char>> {
	let mut frames:Vec<Vec<char>> = Vec::new();

	for i in 1..num_frames {
		let deletion:f64 = 1f64/(i as f64);
		print!("{}",deletion);
		frames.push(shimmer_frame(100, 100, deletion));
	}

	frames
}

fn shimmer_frame(x:i32, y:i32, deletion:f64) -> Vec<char>{
	let mut rng = rand::rng();

	let mut raster:Vec<char> = Vec::new();

	for _i in 0..y {
		for _j in 0..x {
			let is_char = rng.random_bool(deletion);
			if is_char {
				raster.push('#');
			}else{
				raster.push(' ');
			}
		}

		raster.push('\n');
	}

	raster
}