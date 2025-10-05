fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lines = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Gold Rings,",
        "six Geese a Laying,",
        "seven Swans a Swimming,",
        "eight Maids a Milking,",
        "nine Ladies Dancing,",
        "ten Lords a Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,"
    ];

    for day in 0..days.len() {
        println!("On the {} day of Christmas my true love sent to me", days[day]);
        if day == 0 {
            println!("{}", lines[0]);
        } else {
            for line in (1..=day).rev() {
                println!("{}", lines[line]);
            }
            print!("and ");
            println!("{}", lines[0]);
        }
    }
}