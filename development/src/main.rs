use std::io;

fn main() {
    println!("\n\n#######################################################");
    println!("########### Welcome to the Time Calculator! ###########");
    println!("#######################################################\n");
    loop {
        println!("\nEnter 'h' for time difference, 's' to convert (s. -> min.) or 'exit' to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "exit" => break,
            "h" => handle_time_diff(),
            "s" => handle_seconds_conversion(),
            _ => println!("Invalid command. Please try again."),
        }
    }
    println!("\n\n#######################################################");
    println!("########## Thanks to use the Time Calculator! #########");
    println!("#######################################################\n\n");
}

fn handle_time_diff() {
    let get_time = |prompt: &str| -> (u8, u8) {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        parse_time(&format_time_input(&input))
    };
    let (h1, m1) = get_time("\nEnter the first time (hh:mm):");
    let (h2, m2) = get_time("Enter the second time (hh:mm):");
    let (dh, dm) = calculate_time_diff((h1, m1), (h2, m2));
    println!("\n{} hours and {} minutes", dh, dm);
}

fn handle_seconds_conversion() {
    println!("\nEnter the number of seconds:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let seconds: u32 = input.trim().parse().expect("Invalid number");
    let (min, sec) = (seconds / 60, (seconds % 60) as u8);
    println!("\n{} minutes and {} seconds", min, sec);
}

fn format_time_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.contains(':') {
        trimmed.to_string()
    } else {
        format!("{}:00", trimmed)
    }
}

fn parse_time(time: &str) -> (u8, u8) {
    let parts: Vec<&str> = time.split(':').collect();
    (
        parts[0].parse().expect("Invalid hour"),
        parts[1].parse().expect("Invalid minute"),
    )
}

fn calculate_time_diff(start: (u8, u8), end: (u8, u8)) -> (u8, u8) {
    let to_minutes = |(h, m): (u8, u8)| h as u16 * 60 + m as u16;
    let total_diff = to_minutes(end) as i16 - to_minutes(start) as i16;
    let normalized_diff = if total_diff < 0 { total_diff + 1440 } else { total_diff } as u16;
    ((normalized_diff / 60) as u8, (normalized_diff % 60) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_time_diff() {
        assert_eq!(calculate_time_diff((12, 00), (15, 30)), (3, 30));
        assert_eq!(calculate_time_diff((0, 0), (23, 59)), (23, 59));
        assert_eq!(calculate_time_diff((10, 30), (11, 15)), (0, 45));
        assert_eq!(calculate_time_diff((23, 59), (0, 1)), (0, 2));
    }
    #[test]
    fn test_format_time_input() {
        assert_eq!(format_time_input("12"), "12:00");
        assert_eq!(format_time_input("12:30"), "12:30");
        assert_eq!(format_time_input("  21  "), "21:00");
    }
}