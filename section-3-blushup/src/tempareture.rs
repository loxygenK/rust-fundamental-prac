// ~~Use i32 for celsius (negative value is possible)~~
// Use f32 (decimal is possible too, and cannot do math between i32 and f32)
fn to_fahrenheit(celsius: f32) -> f32 {
    return (celsius * 9.0 / 5.0) + 32.0;
}

fn to_celsius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}

#[test]
fn tempareture_test() {
    assert_eq!(to_celsius(50.0), 10.0);
    assert_eq!(to_fahrenheit(50.0), 122.0);

}
