use std::io::Read;

fn mask_byte(byte: &u8, mask_idx: u8) -> (bool, bool) {
    (
        byte >> (6 - (2 * mask_idx)) & 0b10 == 0b10,
        byte >> (6 - (2 * mask_idx)) & 0b01 == 0b01,
    )
}

fn byte_to_index(byte: &u8, mask_idx: u8, line_idx: u8) -> u8 {
    let (left, right) = mask_byte(byte, mask_idx);

    let left_add = if left {
        match line_idx {
            0 => 0b00000001,
            1 => 0b00000010,
            2 => 0b00000100,
            3 => 0b01000000,
            _ => panic!("Invalid line index"),
        }
    } else {
        0
    };

    let right_add = if right {
        match line_idx {
            0 => 0b00001000,
            1 => 0b00010000,
            2 => 0b00100000,
            3 => 0b10000000,
            _ => panic!("Invalid line index"),
        }
    } else {
        0
    };

    left_add | right_add
}

fn u16_to_utf8(byte: u16) -> [u8; 3] {
    let mut utf8 = [0; 3];
    utf8[0] = (byte >> 12) as u8 | 0b11100000;
    utf8[1] = ((byte >> 6) & 0b00111111) as u8 | 0b10000000;
    utf8[2] = (byte & 0b00111111) as u8 | 0b10000000;
    utf8
}

fn braille_to_utf8(braille: u8) -> [u8; 3] {
    u16_to_utf8(0x2800 | braille as u16)
}

fn main() {
    let mut data = Vec::new();

    std::io::stdin()
        .read_to_end(&mut data)
        .expect("Cannot read data");

    let lines = data.chunks(8).collect::<Vec<_>>();

    let lines = lines
        .chunks(4)
        .map(|chunk| {
            let mut braille_values = Vec::new();

            let empty_line = [0u8; 8];
            let empty_line_slice = empty_line.as_slice();

            let line1 = chunk.first().unwrap_or(&empty_line_slice);
            let line2 = chunk.get(1).unwrap_or(&empty_line_slice);
            let line3 = chunk.get(2).unwrap_or(&empty_line_slice);
            let line4 = chunk.get(3).unwrap_or(&empty_line_slice);

            for i in 0..8 {
                for mask_idx in 0..4 {
                    let byte1 = line1.get(i).unwrap_or(&0);
                    let byte2 = line2.get(i).unwrap_or(&0);
                    let byte3 = line3.get(i).unwrap_or(&0);
                    let byte4 = line4.get(i).unwrap_or(&0);

                    let braille = byte_to_index(byte1, mask_idx, 0)
                        | byte_to_index(byte2, mask_idx, 1)
                        | byte_to_index(byte3, mask_idx, 2)
                        | byte_to_index(byte4, mask_idx, 3);

                    braille_values.extend_from_slice(&braille_to_utf8(braille));
                }
                braille_values.push(b' ');
            }

            String::from_utf8(braille_values)
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(idx, s)| format!("{:08x}| {s}", idx * 32))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", lines);
}
