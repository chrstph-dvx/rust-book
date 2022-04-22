fn display_gifts(day: usize) {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for number in (0..day).rev() {
        if day > 1 && number == 0 {
            println!("And {}", &gifts[number].to_lowercase());
        } else {
            println!("{}", &gifts[number]);
        }
    }
}

fn display_day(day: usize) {
    let day_string = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };
    println!("On the {} day of Christmas", day_string);
    println!("My true love sent to me");
}

fn main() {
    for number in 1..13 {
        display_day(number);
        display_gifts(number);
        println!("");
    }
}
