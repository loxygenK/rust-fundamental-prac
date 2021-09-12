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
    let lyrics_segms = LYRICS_SEGMENTS
        .iter()
        .copied()
        .rev()
        .skip(11 - verse)
        .collect::<Vec<&str>>()
        .join("\n");

    // (version 3)
    // [T; N] does NOT implement IntoIterable, but
    // &[T; N] DOES implement. so when .into_iter() is called on [T; N]
    // the '&' is implied and the type of items in iterable becomes &T.
    //
    // Reference: https://qiita.com/harvath/items/b79eaf61e73e79e3fc0f

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
