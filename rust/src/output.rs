use crate::hash::HashBits;
use crate::luck::{calculate_luck_scores, LuckScore};
use chrono::NaiveDate;
use serde::Serialize;
use sha2::{Digest, Sha256};

fn hash_seed_for_display(seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let result = hasher.finalize();
    format!("device:{:02x}{:02x}{:02x}{:02x}", result[0], result[1], result[2], result[3])
}

#[derive(Debug, Serialize)]
pub struct OmikujiResult {
    pub year: u32,
    pub seed: String,
    pub lucky_number: u8,
    pub lucky_hex: String,
    pub lucky_color: String,
    pub lucky_bits: String,
    pub lucky_day: String,
    pub lucky_day_number: u16,
    pub lucky_time: String,
    pub lucky_power_of_2: u8,
    pub lucky_ascii: char,
    pub lucky_logic_gate: String,
    pub lucky_emoji: String,
    pub lucky_direction: String,
    pub lucky_element: String,
    pub lucky_percent: u8,
    pub lucky_latitude: i8,
    pub lucky_longitude: i16,
    pub luck_scores: Vec<LuckScore>,
    pub entropy_check: String,
    pub fingerprint: String,
}

impl OmikujiResult {
    pub fn from_hash(hash: &HashBits, year: u32, seed: &str) -> Self {
        let lucky_number = hash.lucky_number();
        let lucky_hex_val = hash.lucky_hex();
        let lucky_bits_val = hash.lucky_bits();
        let lucky_day_num = hash.lucky_day();
        let lucky_hour = hash.lucky_hour();
        let lucky_minute = hash.lucky_minute();
        let lucky_power_of_2 = hash.lucky_power_of_2();
        let lucky_ascii = hash.lucky_ascii();
        let lucky_logic_gate = hash.lucky_logic_gate().to_string();
        let lucky_emoji = hash.lucky_emoji().to_string();
        let lucky_direction = hash.lucky_direction().to_string();
        let lucky_element = hash.lucky_element().to_string();
        let lucky_percent = hash.lucky_percent();
        let lucky_latitude = hash.lucky_latitude();
        let lucky_longitude = hash.lucky_longitude();
        let scores = hash.luck_scores();
        let entropy = hash.entropy_check();

        let lucky_hex = format!("0x{:02X}", lucky_hex_val);
        let lucky_color = format!("#{:02X}{:02X}{:02X}", lucky_hex_val, lucky_number, (lucky_hex_val.wrapping_add(lucky_number)) / 2);
        let lucky_bits = format!(
            "{:04b} {:04b} {:04b} {:04b}",
            (lucky_bits_val >> 12) & 0xF,
            (lucky_bits_val >> 8) & 0xF,
            (lucky_bits_val >> 4) & 0xF,
            lucky_bits_val & 0xF
        );

        let base_date = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
        let lucky_date = base_date + chrono::Duration::days((lucky_day_num - 1) as i64);
        let lucky_day = format!("{} ({} / 365)", lucky_date.format("%Y-%m-%d"), lucky_day_num);
        let lucky_time = format!("{:02}:{:02}", lucky_hour, lucky_minute);

        let luck_scores = calculate_luck_scores(&scores);
        let entropy_check = format!("0x{:03X}", entropy);
        let fingerprint = hash.hex_string();

        Self {
            year,
            seed: hash_seed_for_display(seed),
            lucky_number,
            lucky_hex,
            lucky_color,
            lucky_bits,
            lucky_day,
            lucky_day_number: lucky_day_num,
            lucky_time,
            lucky_power_of_2,
            lucky_ascii,
            lucky_logic_gate,
            lucky_emoji,
            lucky_direction,
            lucky_element,
            lucky_percent,
            lucky_latitude,
            lucky_longitude,
            luck_scores,
            entropy_check,
            fingerprint,
        }
    }

    pub fn format_text(&self, short: bool, show_seed: bool) -> String {
        let mut output = String::new();

        output.push_str(&format!("🎍 Hash-Omikuji {} 🎍\n\n", self.year));

        output.push_str(&format!("Lucky Number      : {}\n", self.lucky_number));
        output.push_str(&format!("Lucky Hex         : {}\n", self.lucky_hex));
        output.push_str(&format!("Lucky Color       : {}\n", self.lucky_color));
        output.push_str(&format!("Lucky Bits        : {}\n", self.lucky_bits));
        output.push('\n');

        output.push_str(&format!("Lucky Day         : {}\n", self.lucky_day));
        output.push_str(&format!("Lucky Time        : {}\n", self.lucky_time));
        output.push('\n');

        output.push_str(&format!("Lucky Power of 2  : {}\n", self.lucky_power_of_2));
        output.push_str(&format!("Lucky ASCII       : '{}'\n", self.lucky_ascii));
        output.push_str(&format!("Lucky Logic Gate  : {}\n", self.lucky_logic_gate));
        output.push_str(&format!("Lucky Emoji       : {}\n", self.lucky_emoji));
        output.push_str(&format!("Lucky Direction   : {}\n", self.lucky_direction));
        output.push_str(&format!("Lucky Element     : {}\n", self.lucky_element));
        output.push_str(&format!("Lucky Percent     : {}%\n", self.lucky_percent));
        output.push_str(&format!("Lucky Location    : {}°, {}°\n", self.lucky_latitude, self.lucky_longitude));
        output.push('\n');

        output.push_str("Luck Scores :\n");
        let mut sorted_scores: Vec<_> = self.luck_scores.iter().collect();
        sorted_scores.sort_by(|a, b| b.score.cmp(&a.score));

        let display_count = if short { 5 } else { sorted_scores.len() };
        for score in sorted_scores.iter().take(display_count) {
            output.push_str(&format!(
                "{:18}: {:3} ({})\n",
                score.luck_type.name(),
                score.score,
                score.rank.as_str()
            ));
        }
        if show_seed {
            output.push_str(&format!(
                "\n{} | {} | {}\n",
                self.seed,
                &self.fingerprint[..16],
                self.entropy_check
            ));
        }

        output
    }

    pub fn format_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result() -> OmikujiResult {
        let hash = HashBits::from_seed(2026, "test-user");
        OmikujiResult::from_hash(&hash, 2026, "test-user")
    }

    #[test]
    fn test_result_deterministic() {
        let result1 = create_test_result();
        let result2 = create_test_result();
        assert_eq!(result1.fingerprint, result2.fingerprint);
        assert_eq!(result1.lucky_number, result2.lucky_number);
    }

    #[test]
    fn test_result_year() {
        let result = create_test_result();
        assert_eq!(result.year, 2026);
    }

    #[test]
    fn test_result_seed() {
        let result = create_test_result();
        // Seed is now hashed for privacy: "device:{8-char-hex}"
        assert!(result.seed.starts_with("device:"));
        assert_eq!(result.seed.len(), 15); // "device:" + 8 hex chars
    }

    #[test]
    fn test_lucky_hex_format() {
        let result = create_test_result();
        assert!(result.lucky_hex.starts_with("0x"));
        assert_eq!(result.lucky_hex.len(), 4); // "0xXX"
    }

    #[test]
    fn test_lucky_color_format() {
        let result = create_test_result();
        assert!(result.lucky_color.starts_with('#'));
        assert_eq!(result.lucky_color.len(), 7); // "#RRGGBB"
    }

    #[test]
    fn test_lucky_bits_format() {
        let result = create_test_result();
        // Format: "XXXX XXXX XXXX XXXX"
        assert_eq!(result.lucky_bits.len(), 19);
        assert!(result.lucky_bits.chars().all(|c| c == '0' || c == '1' || c == ' '));
    }

    #[test]
    fn test_lucky_day_contains_year() {
        let result = create_test_result();
        assert!(result.lucky_day.contains("2026"));
    }

    #[test]
    fn test_lucky_day_number_range() {
        let result = create_test_result();
        assert!(result.lucky_day_number >= 1 && result.lucky_day_number <= 365);
    }

    #[test]
    fn test_lucky_time_format() {
        let result = create_test_result();
        // Format: "HH:MM"
        assert_eq!(result.lucky_time.len(), 5);
        assert!(result.lucky_time.contains(':'));
    }

    #[test]
    fn test_luck_scores_count() {
        let result = create_test_result();
        assert_eq!(result.luck_scores.len(), 16);
    }

    #[test]
    fn test_entropy_check_format() {
        let result = create_test_result();
        assert!(result.entropy_check.starts_with("0x"));
    }

    #[test]
    fn test_fingerprint_length() {
        let result = create_test_result();
        assert_eq!(result.fingerprint.len(), 64);
    }

    #[test]
    fn test_format_text_contains_header() {
        let result = create_test_result();
        let text = result.format_text(false, false);
        assert!(text.contains("Hash-Omikuji 2026"));
    }

    #[test]
    fn test_format_text_short_mode() {
        let result = create_test_result();
        let text_full = result.format_text(false, false);
        let text_short = result.format_text(true, false);
        // Short mode should be shorter or equal
        assert!(text_short.len() <= text_full.len());
    }

    #[test]
    fn test_format_text_show_seed() {
        let result = create_test_result();
        let text_with_seed = result.format_text(false, true);
        let text_without_seed = result.format_text(false, false);
        assert!(text_with_seed.contains("device:"));
        assert!(text_with_seed.contains("|"));
        assert!(!text_without_seed.contains("device:"));
    }

    #[test]
    fn test_format_json_valid() {
        let result = create_test_result();
        let json = result.format_json();
        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn test_format_json_contains_fields() {
        let result = create_test_result();
        let json = result.format_json();
        assert!(json.contains("\"year\""));
        assert!(json.contains("\"seed\""));
        assert!(json.contains("\"lucky_number\""));
        assert!(json.contains("\"luck_scores\""));
    }

    #[test]
    fn test_lucky_power_of_2() {
        let result = create_test_result();
        let valid_powers = [1, 2, 4, 8, 16, 32, 64, 128];
        assert!(valid_powers.contains(&result.lucky_power_of_2));
    }

    #[test]
    fn test_lucky_ascii() {
        let result = create_test_result();
        assert!(result.lucky_ascii >= ' ' && result.lucky_ascii <= '~');
    }

    #[test]
    fn test_lucky_logic_gate() {
        let result = create_test_result();
        let valid_gates = ["AND", "OR", "XOR", "NOT", "NAND", "NOR", "XNOR", "BUFFER"];
        assert!(valid_gates.contains(&result.lucky_logic_gate.as_str()));
    }

    #[test]
    fn test_lucky_emoji() {
        let result = create_test_result();
        // Just check it's not empty (128 emojis is too many to list)
        assert!(!result.lucky_emoji.is_empty());
    }

    #[test]
    fn test_lucky_direction() {
        let result = create_test_result();
        let valid_directions = ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"];
        assert!(valid_directions.contains(&result.lucky_direction.as_str()));
    }

    #[test]
    fn test_lucky_element() {
        let result = create_test_result();
        let valid_elements = [
            "H (1)", "He (2)", "C (6)", "N (7)", "O (8)", "Na (11)", "Mg (12)", "Al (13)",
            "Si (14)", "Fe (26)", "Cu (29)", "Ag (47)", "Au (79)", "Pt (78)", "Pb (82)", "U (92)",
        ];
        assert!(valid_elements.contains(&result.lucky_element.as_str()));
    }

    #[test]
    fn test_lucky_percent() {
        let result = create_test_result();
        assert!(result.lucky_percent <= 100);
    }

    #[test]
    fn test_lucky_latitude() {
        let result = create_test_result();
        assert!(result.lucky_latitude >= -90 && result.lucky_latitude <= 90);
    }

    #[test]
    fn test_lucky_longitude() {
        let result = create_test_result();
        assert!(result.lucky_longitude >= -180 && result.lucky_longitude <= 180);
    }

    #[test]
    fn test_snapshot_text_output() {
        let result = create_test_result();
        let text = result.format_text(false, true);
        insta::assert_snapshot!(text);
    }

    #[test]
    fn test_snapshot_json_output() {
        let result = create_test_result();
        insta::assert_json_snapshot!(result);
    }

}
