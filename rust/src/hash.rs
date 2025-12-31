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

    /// bit[52..115]: Luck Flags (64bit) - each bit = ON/OFF for each Luck
    pub fn luck_flags(&self) -> u64 {
        self.get_bits(52, 64)
    }

    /// bit[116..243]: Luck Scores (128bit = 8bit x 16)
    pub fn luck_scores(&self) -> [u8; 16] {
        let mut scores = [0u8; 16];
        for i in 0..16 {
            scores[i] = self.get_bits(116 + i * 8, 8) as u8;
        }
        scores
    }

    /// bit[244..255]: Entropy/Checksum (12bit)
    pub fn entropy_check(&self) -> u16 {
        self.get_bits(244, 12) as u16
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
    fn test_lucky_day_range() {
        let hash = HashBits::from_seed(2026, "test");
        let day = hash.lucky_day();
        assert!(day >= 1 && day <= 365);
    }

    #[test]
    fn test_lucky_hour_range() {
        let hash = HashBits::from_seed(2026, "test");
        let hour = hash.lucky_hour();
        assert!(hour < 24);
    }

    #[test]
    fn test_lucky_minute_range() {
        let hash = HashBits::from_seed(2026, "test");
        let minute = hash.lucky_minute();
        assert!(minute < 60);
    }
}
