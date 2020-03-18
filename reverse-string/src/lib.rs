use unicode_segmentation::UnicodeSegmentation as us;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
