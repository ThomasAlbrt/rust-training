pub fn match_statement() {
    println!("Match statement");

    let country_code:i32 = 1999;
    let country = match country_code {
        44 => "UK",
        33 => "FR",
        1..=2000 => "unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}