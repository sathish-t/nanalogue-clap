/// Compute the display width of `text`
///
/// ANSI control sequences do not contribute to the width. Every other character is presumed to
/// occupy one column.
#[inline(never)]
pub(crate) fn display_width(text: &str) -> usize {
    let mut width = 0;

    let mut control_sequence = false;
    let control_terminate: char = 'm';

    for ch in text.chars() {
        if ch.is_ascii_control() {
            control_sequence = true;
        } else if control_sequence && ch == control_terminate {
            control_sequence = false;
            continue;
        }

        if !control_sequence {
            width += ch_width(ch);
        }
    }
    width
}

fn ch_width(_: char) -> usize {
    1
}
