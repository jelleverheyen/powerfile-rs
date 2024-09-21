use unicode_segmentation::UnicodeSegmentation;

pub fn unicode_reverse(text: &str) -> String {
    UnicodeSegmentation::graphemes(text, true)
        .rev()
        .collect::<String>()
}
