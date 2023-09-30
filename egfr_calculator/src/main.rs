use std::io;
use std::io::Write;

fn main() {
    print!("Enter your creatine level in mg/dL: ");
    std::io::stdout().flush().unwrap();
    let mut user_egfr = String::new();
    io::stdin().read_line(&mut user_egfr).expect("Failed to read line");
    let user_egfr: f64 = user_egfr.trim().parse().expect("Please enter a value.");
    
    println!("Accepted {user_egfr} mg/dL");
    println!("Your estimaged GFR based upon 2021 statistics is {:.2}. You are {:.2} points away from dialysis.", ckd_epi_creat_equ(&user_egfr), ckd_epi_creat_equ(&user_egfr) - 20.0);
}

fn ckd_epi_creat_equ(creat: &f64) -> f64 {
    // Based upon data from https://www.kidney.org/content/ckd-epi-creatinine-equation-2021
    142.0 * ((if(creat / 0.9) < 1.0 {creat / 0.9} else {1.0}).powf(-0.302)) * ((if(creat / 0.9) > 1.0 {creat / 0.9} else {1.0}).powf(-1.200)) * 0.9938_f64.powi(21)
}