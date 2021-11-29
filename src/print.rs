pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Ken", "Mars");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} loves to {2}",
        "Ken", "Mars", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Aisha",
        activity = "PUBG-Mobile"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
