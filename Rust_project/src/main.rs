// Declare each topic as a module
mod variable_shadowing;
mod scope;
mod constants;
mod type_aliases;
mod compiler_directives;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║     Rust Learning - Topic Explorer       ║");
    println!("╚══════════════════════════════════════════╝\n");

    variable_shadowing::run();
    scope::run();
    constants::run();
    type_aliases::run();
    compiler_directives::run();

    println!("✅ All topics executed successfully!");
}
