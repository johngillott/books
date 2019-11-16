fn main() {
    let days = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine drummers drummin",
        "ten pipers piping",
        "eleven ladies dancing",
        "twelve lords a leaping",
    ];

    let days_len: usize = 12;

    for number in 0..days_len {
        println!(
            "On the {} day of Christmas,\nMy true love sent to me,",
            ordinal_number_suffix(number + 1)
        );

        for ln in (0..number + 1).rev() {
            // first chorus
            if number == 0 {
                println!("{}.\n", sentence_case(days[ln]));
                break;
            }

            // last line of each chorus after the first
            if ln == 0 {
                println!("And {}.\n", days[ln]);
                break;
            }

            println!("{},", sentence_case(days[ln]));
        }
    }
}

// https://rosettacode.org/wiki/N%27th#Rust
fn ordinal_number_suffix(i: usize) -> String {
    format!(
        "{}{}",
        i,
        match (i % 10, i % 100) {
            (1, 11) | (2, 12) | (3, 13) => "th",
            (1, _) => "st",
            (2, _) => "nd",
            (3, _) => "rd",
            _ => "th",
        }
    )
}

// https://stackoverflow.com/a/38406885
fn sentence_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
