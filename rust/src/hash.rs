use sha2::{Sha256, Digest};

const SALT: &str = "sha-omikuji-2026";

pub struct HashBits {
    bytes: [u8; 32],
}

impl HashBits {
    pub fn from_seed(year: u32, user: &str) -> Self {
        let seed = format!("{}-{}-{}", year, user, SALT);
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Self { bytes }
    }

    pub fn hex_string(&self) -> String {
        self.bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn get_bits(&self, start_bit: usize, num_bits: usize) -> u64 {
        let mut result: u64 = 0;
        for i in 0..num_bits {
            let bit_index = start_bit + i;
            let byte_index = bit_index / 8;
            let bit_offset = 7 - (bit_index % 8);
            if byte_index < 32 {
                let bit = (self.bytes[byte_index] >> bit_offset) & 1;
                result = (result << 1) | (bit as u64);
            }
        }
        result
    }

    /// bit[0..7]: Lucky Number (8bit) -> 0-255
    pub fn lucky_number(&self) -> u8 {
        self.get_bits(0, 8) as u8
    }

    /// bit[8..15]: Lucky Hex (8bit)
    pub fn lucky_hex(&self) -> u8 {
        self.get_bits(8, 8) as u8
    }

    /// bit[16..31]: Lucky Bits (16bit)
    pub fn lucky_bits(&self) -> u16 {
        self.get_bits(16, 16) as u16
    }

    /// bit[32..40]: Lucky Day (9bit) -> 1-365
    pub fn lucky_day(&self) -> u16 {
        let value = self.get_bits(32, 9) as u16;
        (value % 365) + 1
    }

    /// bit[41..45]: Lucky Hour (5bit) -> 0-23
    pub fn lucky_hour(&self) -> u8 {
        let value = self.get_bits(41, 5) as u8;
        value % 24
    }

    /// bit[46..51]: Lucky Minute (6bit) -> 0-59
    pub fn lucky_minute(&self) -> u8 {
        let value = self.get_bits(46, 6) as u8;
        value % 60
    }

    /// bit[52..54]: Lucky Power of 2 (3bit) -> 2^n (1,2,4,8,16,32,64,128)
    pub fn lucky_power_of_2(&self) -> u8 {
        let n = self.get_bits(52, 3) as u8;
        1 << n
    }

    /// bit[55..61]: Lucky ASCII (7bit) -> printable ASCII 32-126 (95 chars)
    pub fn lucky_ascii(&self) -> char {
        let value = self.get_bits(55, 7) as u8;
        let ascii_code = 32 + (value % 95);
        ascii_code as char
    }

    /// bit[62..64]: Lucky Logic Gate (3bit) -> 0-7 maps to gate
    pub fn lucky_logic_gate(&self) -> &'static str {
        let value = self.get_bits(62, 3) as u8;
        match value {
            0 => "AND",
            1 => "OR",
            2 => "XOR",
            3 => "NOT",
            4 => "NAND",
            5 => "NOR",
            6 => "XNOR",
            _ => "BUFFER",
        }
    }

    /// bit[65..192]: Luck Scores (128bit = 8bit x 16)
    pub fn luck_scores(&self) -> [u8; 16] {
        let mut scores = [0u8; 16];
        for i in 0..16 {
            scores[i] = self.get_bits(65 + i * 8, 8) as u8;
        }
        scores
    }

    /// bit[193..204]: Entropy/Checksum (12bit)
    pub fn entropy_check(&self) -> u16 {
        self.get_bits(193, 12) as u16
    }

    /// bit[205..207]: Lucky Emoji (3bit) -> 8 safe emojis
    pub fn lucky_emoji(&self) -> &'static str {
        let value = self.get_bits(205, 3) as u8;
        match value {
            0 => "😀",
            1 => "😎",
            2 => "🎉",
            3 => "🔥",
            4 => "⭐",
            5 => "💡",
            6 => "🎯",
            _ => "🚀",
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let hash1 = HashBits::from_seed(2026, "alice");
        let hash2 = HashBits::from_seed(2026, "alice");
        assert_eq!(hash1.hex_string(), hash2.hex_string());
    }

    #[test]
    fn test_different_users_different_hash() {
        let hash1 = HashBits::from_seed(2026, "alice");
        let hash2 = HashBits::from_seed(2026, "bob");
        assert_ne!(hash1.hex_string(), hash2.hex_string());
    }

    #[test]
    fn test_different_years_different_hash() {
        let hash1 = HashBits::from_seed(2025, "alice");
        let hash2 = HashBits::from_seed(2026, "alice");
        assert_ne!(hash1.hex_string(), hash2.hex_string());
    }

    #[test]
    fn test_lucky_day_range() {
        let hash = HashBits::from_seed(2026, "test");
        let day = hash.lucky_day();
        assert!(day >= 1 && day <= 365);
    }

    #[test]
    fn test_lucky_day_range_many_seeds() {
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let day = hash.lucky_day();
            assert!(day >= 1 && day <= 365, "Day out of range: {}", day);
        }
    }

    #[test]
    fn test_lucky_hour_range() {
        let hash = HashBits::from_seed(2026, "test");
        let hour = hash.lucky_hour();
        assert!(hour < 24);
    }

    #[test]
    fn test_lucky_hour_range_many_seeds() {
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let hour = hash.lucky_hour();
            assert!(hour < 24, "Hour out of range: {}", hour);
        }
    }

    #[test]
    fn test_lucky_minute_range() {
        let hash = HashBits::from_seed(2026, "test");
        let minute = hash.lucky_minute();
        assert!(minute < 60);
    }

    #[test]
    fn test_lucky_minute_range_many_seeds() {
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let minute = hash.lucky_minute();
            assert!(minute < 60, "Minute out of range: {}", minute);
        }
    }

    #[test]
    fn test_lucky_number_range() {
        let hash = HashBits::from_seed(2026, "test");
        let num = hash.lucky_number();
        assert!(num <= 255);
    }

    #[test]
    fn test_luck_scores_count() {
        let hash = HashBits::from_seed(2026, "test");
        let scores = hash.luck_scores();
        assert_eq!(scores.len(), 16);
    }

    #[test]
    fn test_hex_string_length() {
        let hash = HashBits::from_seed(2026, "test");
        let hex = hash.hex_string();
        assert_eq!(hex.len(), 64); // 32 bytes * 2 hex chars
    }

    #[test]
    fn test_hex_string_valid_chars() {
        let hash = HashBits::from_seed(2026, "test");
        let hex = hash.hex_string();
        for ch in hex.chars() {
            assert!(ch.is_ascii_hexdigit(), "Invalid hex char: {}", ch);
        }
    }

    #[test]
    fn test_entropy_check_range() {
        let hash = HashBits::from_seed(2026, "test");
        let entropy = hash.entropy_check();
        assert!(entropy <= 0xFFF); // 12 bits max
    }

    #[test]
    fn test_lucky_power_of_2_range() {
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let power = hash.lucky_power_of_2();
            assert!(
                power == 1 || power == 2 || power == 4 || power == 8 ||
                power == 16 || power == 32 || power == 64 || power == 128,
                "Power of 2 not valid: {}", power
            );
        }
    }

    #[test]
    fn test_lucky_ascii_range() {
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let ch = hash.lucky_ascii();
            assert!(ch >= ' ' && ch <= '~', "ASCII not printable: {:?}", ch);
        }
    }

    #[test]
    fn test_lucky_logic_gate_valid() {
        let valid_gates = ["AND", "OR", "XOR", "NOT", "NAND", "NOR", "XNOR", "BUFFER"];
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let gate = hash.lucky_logic_gate();
            assert!(valid_gates.contains(&gate), "Invalid gate: {}", gate);
        }
    }

    #[test]
    fn test_lucky_emoji_valid() {
        let valid_emojis = ["😀", "😎", "🎉", "🔥", "⭐", "💡", "🎯", "🚀"];
        for i in 0..100 {
            let seed = format!("test-{}", i);
            let hash = HashBits::from_seed(2026, &seed);
            let emoji = hash.lucky_emoji();
            assert!(valid_emojis.contains(&emoji), "Invalid emoji: {}", emoji);
        }
    }

}
