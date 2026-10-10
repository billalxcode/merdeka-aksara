//! Greedy line breaking on spaces with deterministic fallback.

use crate::measure::TextMeasurer;

/// Break `text` into lines fitting `max_width_pt`.
///
/// Words stay whole; the separating space is not kept at the start
/// of a line. A word wider than `max_width_pt` is cut per char.
/// Hard newlines force a break. Runs of spaces collapse to one.
pub fn break_greedy(
    text: &str,
    measurer: &dyn TextMeasurer,
    size_pt: f32,
    bold: bool,
    italic: bool,
    max_width_pt: f64,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for forced in text.split('\n') {
        // Collapse ASCII whitespace runs to single spaces, drop empties.
        let words: Vec<&str> = forced.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        let mut current = String::new();
        for word in words {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if measurer.width_pt(&candidate, size_pt, bold, italic) <= max_width_pt {
                current = candidate;
            } else if current.is_empty() {
                // Single word wider than the line: cut per char.
                let mut piece = String::new();
                for ch in word.chars() {
                    let next = format!("{piece}{ch}");
                    if !piece.is_empty()
                        && measurer.width_pt(&next, size_pt, bold, italic) > max_width_pt
                    {
                        lines.push(piece);
                        piece = ch.to_string();
                    } else {
                        piece = next;
                    }
                }
                current = piece;
            } else {
                lines.push(current);
                // Retry the word on a fresh line (it may still need cutting).
                if measurer.width_pt(word, size_pt, bold, italic) <= max_width_pt {
                    current = word.to_owned();
                } else {
                    let mut piece = String::new();
                    for ch in word.chars() {
                        let next = format!("{piece}{ch}");
                        if !piece.is_empty()
                            && measurer.width_pt(&next, size_pt, bold, italic) > max_width_pt
                        {
                            lines.push(piece);
                            piece = ch.to_string();
                        } else {
                            piece = next;
                        }
                    }
                    current = piece;
                }
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::FixedMeasurer;

    fn measurer() -> FixedMeasurer {
        FixedMeasurer::default()
    }

    #[test]
    fn short_text_is_one_line() {
        let m = measurer();
        assert_eq!(
            break_greedy("hi", &m, 12.0, false, false, 451.28),
            vec!["hi"]
        );
    }

    #[test]
    fn long_text_wraps_within_width() {
        let m = measurer();
        let lines = break_greedy(
            "satu dua tiga empat lima enam tujuh delapan",
            &m,
            12.0,
            false,
            false,
            72.0,
        );
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(m.width_pt(line, 12.0, false, false) <= 72.0 + 1e-9);
        }
    }

    #[test]
    fn giant_word_is_cut_per_char() {
        let m = measurer();
        // 12pt * 0.6 = 7.2 pt per char; width 20 fits 2 chars.
        let lines = break_greedy("abcdefghij", &m, 12.0, false, false, 20.0);
        assert!(lines.len() > 1);
        assert!(lines.concat().replace(' ', "") == "abcdefghij");
    }

    #[test]
    fn empty_string_has_no_lines() {
        let m = measurer();
        assert!(break_greedy("", &m, 12.0, false, false, 100.0).is_empty());
    }

    #[test]
    fn newline_forces_break() {
        let m = measurer();
        assert_eq!(
            break_greedy("ab\ncd", &m, 12.0, false, false, 451.28),
            vec!["ab", "cd"]
        );
    }

    #[test]
    fn double_spaces_collapse() {
        let m = measurer();
        assert_eq!(
            break_greedy("a  b   c", &m, 12.0, false, false, 451.28),
            vec!["a b c"]
        );
    }
}
