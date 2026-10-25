const TAX_RATE: f64 = 0.075;

pub fn run() {
    println!("===== Constants =====\n");

    let income: f64 = 755_000.0;
    let income_tax = income * TAX_RATE;
    let income_after_tax = income - income_tax;

    println!("Yearly Salary:       ₹{:.2}", income);
    println!("Tax Rate:            {:.1}%", TAX_RATE * 100.0);
    println!("Income Tax:          ₹{:.2}", income_tax);
    println!("Take-Home After Tax: ₹{:.2}", income_after_tax);

    println!();
    println!("Key takeaway: `const` values are global, compile-time, and never change.");
    println!("Use SCREAMING_SNAKE_CASE for constant names (e.g., TAX_RATE, MAX_SIZE).");
    println!();
}
