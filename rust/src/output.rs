use crate::art::generate_omikuji_art;
use crate::hash::HashBits;
use crate::luck::{calculate_luck_scores, LuckScore, LuckType};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OmikujiResult {
    pub year: u32,
    pub user: String,
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
    pub fn from_hash(hash: &HashBits, year: u32, user: &str) -> Self {
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
            user: user.to_string(),
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
            output.push_str(&format!("\nFingerprint       : {}\n", self.fingerprint));
        }

        output
    }

    pub fn format_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
