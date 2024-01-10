use fundamentals_week4::colors::{Color, ColorString};

#[test]
fn test_red_coloring() {
    // use new method to create a new instance of ColorString
    let mut color_string: ColorString = ColorString::new(Color::Red, "Red".to_string());
    // let mut color_string = ColorString {
    //     color: Color::Red,
    //     string: "Red".to_string(),
    //     colorized: "".to_string(),
    // };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}
