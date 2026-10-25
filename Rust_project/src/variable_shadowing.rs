pub fn run() {
    println!("===== Variable Shadowing =====\n");

    let grams_of_protein = "100.345";
    println!("As string (&str):    \"{}\"", grams_of_protein);

    let grams_of_protein = 100.345;
    println!("As float  (f64):     {}", grams_of_protein);

    let grams_of_protein = 80;
    println!("As integer (i32):    {}", grams_of_protein);

    println!();
    println!("Key takeaway: Each `let` creates a NEW variable.");
    println!("The type is determined by the latest binding — previous versions become invalid.");
    println!();
}
