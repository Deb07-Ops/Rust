#[allow(unused_variables)]
pub fn run() {
    println!("===== Compiler Directives (Attributes) =====\n");

    let unused_var = 42; // no warning due to #[allow(unused_variables)]
    let used_var = 100;
    println!("used_var = {}", used_var);

    println!();
    println!("Key takeaway: Attributes like `#[allow(...)]` control compiler behavior.");
    println!("`#[...]` applies to the next item; `#![...]` applies to the whole file.");
    println!();
}
