type Meters = i32;

pub fn run() {
    println!("===== Type Aliases =====\n");

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = mile_race_length * 2;

    println!("1-mile race length: {} meters", mile_race_length);
    println!("2-mile race length: {} meters", two_mile_race_length);

    let total: i32 = mile_race_length + two_mile_race_length;
    println!("Combined distance:  {} meters", total);

    println!();
    println!("Key takeaway: `type Meters = i32;` makes code self-documenting.");
    println!("The alias is purely cosmetic — the compiler sees it as the original type.");
    println!();
}
