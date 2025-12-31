/// 1D randomart generator inspired by SSH randomart
/// Uses 2-bit steps from SHA-256 hash to create a visual fingerprint

pub fn generate_omikuji_art(hash_bytes: &[u8; 32]) -> String {
    const WIDTH: usize = 16;
    let mut grid = [0u8; WIDTH];
    let mut position: i32 = 0;
    let start_pos = 0;

    // Process all 256 bits as 2-bit pairs (128 steps)
    for byte in hash_bytes.iter() {
        for shift in (0..8).step_by(2) {
            let bits = (byte >> (6 - shift)) & 0b11;
            let movement: i32 = match bits {
                0b00 => 0,  // stay
                0b01 => 1,  // right
                0b10 => 2,  // right x2
                0b11 => 3,  // right x3
                _ => unreachable!(),
            };

            position += movement;

            // Wrap around (modulo)
            position = position.rem_euclid(WIDTH as i32);

            // Increment visit count (saturating at 255)
            grid[position as usize] = grid[position as usize].saturating_add(1);
        }
    }

    let end_pos = position as usize;

    // Build the art string
    let mut art = String::with_capacity(WIDTH);
    for (i, &count) in grid.iter().enumerate() {
        let ch = if i == start_pos && i == end_pos {
            'X' // Start and end at same position
        } else if i == start_pos {
            'S'
        } else if i == end_pos {
            'E'
        } else if count == 0 {
            '.'
        } else if count == 1 {
            '+'
        } else {
            '#'
        };
        art.push(ch);
    }

    art
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Sha256, Digest};

    #[test]
    fn test_art_length() {
        let hash = [0u8; 32];
        let art = generate_omikuji_art(&hash);
        assert_eq!(art.len(), 16);
    }

    #[test]
    fn test_art_deterministic() {
        let mut hasher = Sha256::new();
        hasher.update(b"test-seed");
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);

        let art1 = generate_omikuji_art(&hash);
        let art2 = generate_omikuji_art(&hash);
        assert_eq!(art1, art2);
    }

    #[test]
    fn test_art_has_start_or_end() {
        let mut hasher = Sha256::new();
        hasher.update(b"some-user");
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);

        let art = generate_omikuji_art(&hash);
        assert!(art.contains('S') || art.contains('E') || art.contains('X'));
    }

    #[test]
    fn test_different_hashes_different_art() {
        let mut hasher1 = Sha256::new();
        hasher1.update(b"alice");
        let result1 = hasher1.finalize();
        let mut hash1 = [0u8; 32];
        hash1.copy_from_slice(&result1);

        let mut hasher2 = Sha256::new();
        hasher2.update(b"bob");
        let result2 = hasher2.finalize();
        let mut hash2 = [0u8; 32];
        hash2.copy_from_slice(&result2);

        let art1 = generate_omikuji_art(&hash1);
        let art2 = generate_omikuji_art(&hash2);
        assert_ne!(art1, art2);
    }
}
