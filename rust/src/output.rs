use crate::art::generate_omikuji_art;
use crate::hash::HashBits;
use crate::luck::{calculate_luck_scores, LuckScore, LuckType};
use chrono::NaiveDate;
use serde::Serialize;

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
    pub luck_scores: Vec<LuckScore>,
    pub entropy_check: String,
    pub fingerprint: String,
    pub omikuji_art: String,
}

impl OmikujiResult {
    pub fn from_hash(hash: &HashBits, year: u32, seed: &str) -> Self {
        let lucky_number = hash.lucky_number();
        let lucky_hex_val = hash.lucky_hex();
        let lucky_bits_val = hash.lucky_bits();
        let lucky_day_num = hash.lucky_day();
        let lucky_hour = hash.lucky_hour();
        let lucky_minute = hash.lucky_minute();
        let flags = hash.luck_flags();
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

        let luck_scores = calculate_luck_scores(&scores, flags);
        let entropy_check = format!("0x{:03X}", entropy);
        let fingerprint = hash.hex_string();
        let omikuji_art = generate_omikuji_art(hash.raw_bytes());

        Self {
            year,
            seed: seed.to_string(),
            lucky_number,
            lucky_hex,
            lucky_color,
            lucky_bits,
            lucky_day,
            lucky_day_number: lucky_day_num,
            lucky_time,
            luck_scores,
            entropy_check,
            fingerprint,
            omikuji_art,
        }
    }

    pub fn format_text(&self, short: bool, show_seed: bool) -> String {
        let mut output = String::new();

        output.push_str(&format!("🎍 SHA-Omikuji {} 🎍\n\n", self.year));

        output.push_str(&format!("Lucky Number      : {}\n", self.lucky_number));
        output.push_str(&format!("Lucky Hex         : {}\n", self.lucky_hex));
        output.push_str(&format!("Lucky Color       : {}\n", self.lucky_color));
        output.push_str(&format!("Lucky Bits        : {}\n", self.lucky_bits));
        output.push('\n');

        output.push_str(&format!("Lucky Day         : {}\n", self.lucky_day));
        output.push_str(&format!("Lucky Time        : {}\n", self.lucky_time));
        output.push('\n');

        output.push_str("Active Luck Flags :\n");
        let flag_line: Vec<String> = LuckType::ALL
            .iter()
            .take(if short { 5 } else { 16 })
            .map(|lt| {
                let score = self.luck_scores.iter().find(|s| s.luck_type == *lt).unwrap();
                let mark = if score.active { "✔" } else { "✖" };
                format!("{} {}", mark, lt.name().replace(" Luck", ""))
            })
            .collect();
        output.push_str(&format!("{}\n\n", flag_line.join("  ")));

        output.push_str("Luck Scores :\n");
        let mut active_scores: Vec<_> = self.luck_scores.iter().filter(|s| s.active).collect();
        active_scores.sort_by(|a, b| b.score.cmp(&a.score));

        let display_count = if short { 5 } else { active_scores.len() };
        for score in active_scores.iter().take(display_count) {
            output.push_str(&format!(
                "{:18}: {:3} ({})\n",
                score.luck_type.name(),
                score.score,
                score.rank.as_str()
            ));
        }
        output.push('\n');

        output.push_str(&format!("Entropy Check     : OK ({})\n", self.entropy_check));

        output.push_str("\n[ Omikuji Art ]\n");
        output.push_str(&format!("{}\n", self.omikuji_art));

        if show_seed {
            output.push_str(&format!("\nSeed              : {}\n", self.seed));
            output.push_str(&format!("Fingerprint       : {}\n", self.fingerprint));
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
        assert_eq!(result.seed, "test-user");
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
    fn test_omikuji_art_length() {
        let result = create_test_result();
        assert_eq!(result.omikuji_art.len(), 16);
    }

    #[test]
    fn test_format_text_contains_header() {
        let result = create_test_result();
        let text = result.format_text(false, false);
        assert!(text.contains("SHA-Omikuji 2026"));
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
        assert!(text_with_seed.contains("Seed"));
        assert!(text_with_seed.contains("Fingerprint"));
        assert!(!text_without_seed.contains("Seed"));
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
}
