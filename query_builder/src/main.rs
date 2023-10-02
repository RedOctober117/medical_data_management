use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    println!("Welcome to the medical_db query builder. To begin, select the query you'd like to build.");
    println!("1. medical_report_line_item\n2. medical_report");

    print!(":");

    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_selection: i32 = user_input.trim().parse().expect("Error parsing value");

    if user_selection == 1 {
        println!("{}", line_item_builder());
    } else {
        println!("{}", medical_report_builder());
    };
}

fn line_item_builder() -> String {
    let mut user_input = String::new();

    print!("Report ID: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let report_id: i64 = user_input.trim().parse().expect("Error parsing value");

    user_input = String::new();

    print!("Value Type: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let value_type: i64 = user_input.trim().parse().expect("Error parsing value");

    user_input = String::new();

    print!("Value Unit: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let value_unit: i64 = user_input.trim().parse().expect("Error parsing value");

    user_input = String::new();

    print!("Value Measurement: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let value_measurement: f64 = user_input.trim().parse().expect("Error parsing value");

    format!("INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement VALUES ({report_id}), ({value_type}), ({value_unit}), ({value_measurement});")
}

fn medical_report_builder() -> String {
    let mut user_input = String::new();

    print!("Report Date (YYY-MM-DD): ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let report_date: String = user_input.trim().parse().expect("Error parsing value");

    user_input = String::new();

    print!("Report Provider: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let report_provider: i64 = user_input.trim().parse().expect("Error parsing value");

    user_input = String::new();

    print!("Report Type: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let report_type: i64 = user_input.trim().parse().expect("Error parsing value");

    format!("INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement VALUES (\"{report_date}\"), ({report_provider}), ({report_type});")
}
