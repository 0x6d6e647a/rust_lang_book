const GIFTS: &'static [&'static str] = &[
    "",
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const ORDINAL_ADVERBS: &'static [&'static str] = &[
    "", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
    "tenth", "eleventh", "twelfth",
];

fn opening_line(day: u8) -> String {
    format!(
        "On the {} day of Christmas, my true love sent to me",
        ORDINAL_ADVERBS[usize::from(day)]
    )
}

fn verse(day: u8) -> String {
    let mut verse = Vec::new();

    verse.push(format!("[Verse {day}]"));
    verse.push(opening_line(day));

    for nday in (1..=day).rev() {
        verse.push(GIFTS[usize::from(nday)].to_string());
    }

    if verse.len() > 3 {
        let last = verse.pop().unwrap();
        let prev = verse.pop().unwrap();
        verse.push(format!("{prev} and"));
        verse.push(last);
    }

    verse.join("\n")
}

fn main() {
    for day in 1u8..=12 {
        println!("{}\n", verse(day));
    }
}
