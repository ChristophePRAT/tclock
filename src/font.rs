pub const HEIGHT: u16 = 5;

pub fn get_digit(c: char) -> &'static [&'static str; 5] {
    match c {
        '0' => &[
            "█████",
            "█   █",
            "█   █",
            "█   █",
            "█████",
        ],
        '1' => &[
            "█",
            "█",
            "█",
            "█",
            "█",
        ],
        '2' => &[
            "█████",
            "    █",
            "█████",
            "█    ",
            "█████",
        ],
        '3' => &[
            "█████",
            "    █",
            "█████",
            "    █",
            "█████",
        ],
        '4' => &[
            "█   █",
            "█   █",
            "█████",
            "    █",
            "    █",
        ],
        '5' => &[
            "█████",
            "█    ",
            "█████",
            "    █",
            "█████",
        ],
        '6' => &[
            "█████",
            "█    ",
            "█████",
            "█   █",
            "█████",
        ],
        '7' => &[
            "█████",
            "    █",
            "    █",
            "    █",
            "    █",
        ],
        '8' => &[
            "█████",
            "█   █",
            "█████",
            "█   █",
            "█████",
        ],
        '9' => &[
            "█████",
            "█   █",
            "█████",
            "    █",
            "█████",
        ],
        ':' => &[
            "  ",
            "██",
            "  ",
            "██",
            "  ",
        ],
        'A' => &[
            " ███ ",
            "█   █",
            "█████",
            "█   █",
            "█   █",
        ],
        'P' => &[
            "████ ",
            "█   █",
            "████ ",
            "█    ",
            "█    ",
        ],
        'M' => &[
            "█   █",
            "██ ██",
            "█ █ █",
            "█   █",
            "█   █",
        ],
        ' ' => &[
            " ",
            " ",
            " ",
            " ",
            " ",
        ],
        _ => &[
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ],
    }
}

pub fn get_width(c: char) -> u16 {
    get_digit(c)[0].chars().count() as u16
}

pub fn get_slot_width(c: char) -> u16 {
    match c {
        '0'..='9' | 'A' | 'P' | 'M' => 5,
        ':' => 2,
        ' ' => 1,
        _ => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_dimensions() {
        let chars = "0123456789: APM";
        for c in chars.chars() {
            let pattern = get_digit(c);
            assert_eq!(pattern.len(), HEIGHT as usize);
            let width = get_width(c);
            for row in pattern.iter() {
                assert_eq!(row.chars().count(), width as usize);
            }
        }
    }
}
