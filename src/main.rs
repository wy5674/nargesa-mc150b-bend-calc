use std::io;

fn parse_measurement(input: &str) -> f32 {
    let mut feet: f32 = 0.0;
    let mut inches: f32 = 0.0;
    let mut fraction: f32 = 0.0;

    let mut s = input.trim();

    // Parse feet: look for '
    if let Some(pos) = s.find('\'') {
        let (f, rest) = s.split_at(pos);
        feet = f.parse::<f32>().unwrap_or(0.0);
        s = &rest[1..]; // skip '
    }

    // Parse inches: look for "
    if let Some(pos) = s.find('"') {
        let (i, rest) = s.split_at(pos);
        inches = i.parse::<f32>().unwrap_or(0.0);
        s = &rest[1..]; // skip "
    }

    // Parse fraction: look for a/b
    if let Some((num, den)) = s.split_once('/') {
        let n = num.trim().parse::<f32>().unwrap_or(0.0);
        let d = den.trim().parse::<f32>().unwrap_or(1.0);
        fraction = n / d;
    }

    // Convert everything to inches
    feet * 12.0 + inches + fraction
}

fn main() {
    loop {
        //finding radius
        println!("Enter Width");
        let mut w_input = String::new();
        io::stdin()
            .read_line(&mut w_input)
            .expect("Error");
        let w: f32 = parse_measurement(&w_input);

        println!("Enter Height");
        let mut height_input = String::new();
        io::stdin()
            .read_line(&mut height_input)
            .expect("Error");
        let height:f32 = parse_measurement(&height_input);

        let a: f32 = height / 2.0;
        let w_2: f32 = w * w; 
        let h_8: f32 = height * 8.0;
        let b: f32 = w_2 / h_8;
        let radius:f32 = a + b;
        println!("--------------------------------");
        println!("R = {}", radius); 

        //depth calc
        let r_2: f32 = radius * radius;
        let c: f32 = r_2 - 20.25;
        let sqr:f32 = c.sqrt();
        let h: f32 = radius - sqr;
        let h_cm: f32 = h * 2.54;
        println!("depth = {}", h_cm);
        println!("--------------------------------");



    }
    
}
