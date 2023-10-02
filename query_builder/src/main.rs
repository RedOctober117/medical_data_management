use std::io;
use std::io::Write;

fn main() {
    loop {
        let exit_condition = String::from("Q");
        let user_input = query_user("Welcome to the medical_db query builder. To begin, select the query you'd like to build.\n1. medical_report_line_item\n2. medical_report\n : ");
        // Trim user_input to drop the new line character at the end
        if user_input.trim().to_uppercase().eq(&exit_condition) {
            break;
        } else {
            let user_selection = parse_user_query(user_input);
            match user_selection {
                1 => println!("{}", line_item_builder()),
                2 => println!("{}", medical_report_builder()),
                i32::MIN..=0_i32 | 3_i32..=i32::MAX => todo!(),
            };
        }
    }
    println!("Now exiting query builder...");
}

fn line_item_builder() -> String {
    let report_id: i32 = parse_user_query(query_user("Report ID: "));
    let value_type: i32 = parse_user_query(query_user("Value Type: "));
    let value_unit: i32 = parse_user_query(query_user("Value Unit: "));
    let value_measurement: i32 = parse_user_query(query_user("Value Measurement: "));

    format!("INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement VALUES\n\t({report_id}),\n\t({value_type}),\n\t({value_unit}),\n\t({value_measurement});\n\n")
}

fn medical_report_builder() -> String {
    let report_date: String = query_user("Report Date (YYY-MM-DD): ").trim().to_string();
    let report_provider: i32 = parse_user_query(query_user("Report Provider: "));
    let report_type: i32 = parse_user_query(query_user("Report Type: "));

    format!("INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement VALUES\n\t(\"{report_date}\"),\n\t({report_provider}),\n\t({report_type});\n\n")
}

fn query_user(message: &str) -> String {
    let mut user_input = String::new();
    loop {
        print!("{}", message);
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        if user_input != "" {
            break;
        }
    }
    user_input
}

fn parse_user_query(input: String) -> i32 {
    let mut internal_input = input;
    // let output: Result<i32, _> = input.trim().parse();
    loop {
        let _output = match internal_input.trim().parse() {
            Ok(output) => {
                return output;
            },
            Err(_) => {
                internal_input = query_user("Enter a valid number: ");
            },
        };
    }
}
