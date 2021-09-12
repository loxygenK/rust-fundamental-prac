const LYRICS_SEGMENTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn the_twelve_days_of_christmas(verse: usize) -> String {
    // Is this good? I don't know
    let lyrics_segms = LYRICS_SEGMENTS
        .iter()
        .rev()
        .skip(11 - verse)
        .map(|f| f.to_owned())
        .collect::<Vec<&str>>()
        .join("\n");

    // [T; N] does NOT implement IntoIterable, but
    // &[T; N] DOES implement. so when .into_iter() is called on [T; N]
    // the '&' is implied and the type of items in iterable becomes &T.
    // (This is why to_owned() is used but I'm not sure this is good practice)
    //
    // Reference: https://t.co/NHsbsiNPur?amp=1

    format!("On the twelfth day of Christmas, my true love sent to me\n{}\n", lyrics_segms)
}

#[test]
fn lyrics_test() {
    assert_eq!(
        the_twelve_days_of_christmas(4),
        include_str!("../asset_test/the_twelve_days_of_christmas_vers_5")
    );
    assert_eq!(
        the_twelve_days_of_christmas(11),
        include_str!("../asset_test/the_twelve_days_of_christmas_vers_12")
    );
}
