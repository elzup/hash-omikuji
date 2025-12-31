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
                0b00 => 0, // stay
                0b01 => 1, // right
                0b10 => 2, // right x2
                0b11 => 3, // right x3
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
    use sha2::{Digest, Sha256};

    fn make_hash(seed: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    #[test]
    fn test_art_length() {
        let hash = [0u8; 32];
        let art = generate_omikuji_art(&hash);
        assert_eq!(art.len(), 16);
    }

    #[test]
    fn test_art_deterministic() {
        let hash = make_hash(b"test-seed");
        let art1 = generate_omikuji_art(&hash);
        let art2 = generate_omikuji_art(&hash);
        assert_eq!(art1, art2);
    }

    #[test]
    fn test_art_has_start_or_end() {
        let hash = make_hash(b"some-user");
        let art = generate_omikuji_art(&hash);
        assert!(art.contains('S') || art.contains('E') || art.contains('X'));
    }

    #[test]
    fn test_different_hashes_different_art() {
        let hash1 = make_hash(b"alice");
        let hash2 = make_hash(b"bob");
        let art1 = generate_omikuji_art(&hash1);
        let art2 = generate_omikuji_art(&hash2);
        assert_ne!(art1, art2);
    }

    #[test]
    fn test_all_zeros_produces_x() {
        // All zeros = all "stay" moves, position stays at 0
        let hash = [0u8; 32];
        let art = generate_omikuji_art(&hash);
        assert!(
            art.starts_with('X'),
            "Expected X at start for all-zero hash, got: {}",
            art
        );
    }

    #[test]
    fn test_art_contains_only_valid_chars() {
        let hash = make_hash(b"random-test-seed");
        let art = generate_omikuji_art(&hash);
        for ch in art.chars() {
            assert!(
                matches!(ch, 'S' | 'E' | 'X' | '.' | '+' | '#'),
                "Invalid character in art: {}",
                ch
            );
        }
    }

    #[test]
    fn test_art_many_seeds() {
        // Test with many different seeds to ensure no panics
        for i in 0..100 {
            let seed = format!("test-seed-{}", i);
            let hash = make_hash(seed.as_bytes());
            let art = generate_omikuji_art(&hash);
            assert_eq!(art.len(), 16);
        }
    }

    #[test]
    fn test_art_start_position() {
        // Start position is always 0, so 'S' or 'X' should be at position 0
        let hash = make_hash(b"check-start");
        let art = generate_omikuji_art(&hash);
        let first_char = art.chars().next().unwrap();
        assert!(
            first_char == 'S' || first_char == 'X',
            "First char should be S or X, got: {}",
            first_char
        );
    }
}
