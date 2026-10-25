pub fn run() {
    println!("===== Scope =====\n");

    let coffee = 6.75;
    println!("[outer] coffee = {}", coffee);

    {
        let coffee = 5.0; // shadows outer `coffee`
        println!("[inner] coffee = {}", coffee);
    } // inner `coffee` dropped here

    println!("[outer] coffee = {} (restored after inner scope ends)", coffee);

    println!();
    println!("Key takeaway: Variables live only within their `{{ }}` block.");
    println!("Shadowing inside a nested scope does NOT affect the outer variable.");
    println!();
}
