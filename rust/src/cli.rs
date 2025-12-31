use clap::Parser;
use chrono::{Datelike, Local};
use gethostname::gethostname;
use std::env;

fn get_default_seed() -> String {
    let hostname = gethostname().to_string_lossy().to_string();
    let username = env::var("USER").unwrap_or_else(|_| "anonymous".to_string());
    format!("{}@{}", username, hostname)
}

#[derive(Parser, Debug)]
#[command(name = "hash-omikuji")]
#[command(author = "elzup")]
#[command(version = "0.1.0")]
#[command(about = "SHA-256 based deterministic fortune telling CLI")]
#[command(long_about = "A deterministic 'omikuji' (fortune slip) generator using SHA-256.\nThis command can only be executed on January 1st.\nSame input always produces the same result.")]
pub struct Args {
    /// Force execution for a specific year (bypasses January 1st restriction)
    #[arg(long)]
    pub force_year: Option<u32>,

    /// Custom seed string (defaults to username@hostname)
    #[arg(long, short)]
    pub seed: Option<String>,

    /// Output as JSON
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Show only top 5 luck scores
    #[arg(long, default_value_t = false)]
    pub short: bool,

    /// Show seed and fingerprint in output
    #[arg(long, default_value_t = false)]
    pub show_seed: bool,

    /// Override current date for testing (format: YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,
}

impl Args {
    pub fn get_seed(&self) -> String {
        self.seed.clone().unwrap_or_else(get_default_seed)
    }

    pub fn get_year(&self) -> u32 {
        self.force_year.unwrap_or_else(|| Local::now().year() as u32)
    }

    pub fn is_january_first(&self) -> bool {
        if let Some(ref date_str) = self.date {
            if let Some((_, rest)) = date_str.split_once('-') {
                if let Some((month, day)) = rest.split_once('-') {
                    return month == "01" && day == "01";
                }
            }
            return false;
        }

        let now = Local::now();
        now.month() == 1 && now.day() == 1
    }

    pub fn can_execute(&self) -> Result<bool, &'static str> {
        if self.is_january_first() {
            Ok(false)  // No warning needed
        } else if self.force_year.is_some() {
            Ok(true)   // Warning needed
        } else {
            Err("This command can only be executed on January 1st.\nUse --force-year <YYYY> to override.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_january_first_detection() {
        let args = Args {
            force_year: None,
            seed: Some("test".to_string()),
            json: false,
            short: false,
            show_seed: false,
            date: Some("2026-01-01".to_string()),
        };
        assert!(args.is_january_first());

        let args = Args {
            force_year: None,
            seed: Some("test".to_string()),
            json: false,
            short: false,
            show_seed: false,
            date: Some("2026-07-15".to_string()),
        };
        assert!(!args.is_january_first());
    }

    #[test]
    fn test_can_execute_with_force_year() {
        let args = Args {
            force_year: Some(2026),
            seed: Some("test".to_string()),
            json: false,
            short: false,
            show_seed: false,
            date: Some("2026-07-15".to_string()),
        };
        assert!(args.can_execute().is_ok());
        assert_eq!(args.get_year(), 2026);
    }

    #[test]
    fn test_cannot_execute_without_force_year() {
        let args = Args {
            force_year: None,
            seed: Some("test".to_string()),
            json: false,
            short: false,
            show_seed: false,
            date: Some("2026-07-15".to_string()),
        };
        assert!(args.can_execute().is_err());
    }

    #[test]
    fn test_get_seed_custom() {
        let args = Args {
            force_year: Some(2026),
            seed: Some("custom-seed".to_string()),
            json: false,
            short: false,
            show_seed: false,
            date: None,
        };
        assert_eq!(args.get_seed(), "custom-seed");
    }

    #[test]
    fn test_get_seed_default() {
        let args = Args {
            force_year: None,
            seed: None,
            json: false,
            short: false,
            show_seed: false,
            date: None,
        };
        let seed = args.get_seed();
        assert!(seed.contains('@'));
    }

    #[test]
    fn test_get_year_with_force() {
        let args = Args {
            force_year: Some(2030),
            seed: None,
            json: false,
            short: false,
            show_seed: false,
            date: None,
        };
        assert_eq!(args.get_year(), 2030);
    }

    #[test]
    fn test_get_year_default() {
        let args = Args {
            force_year: None,
            seed: None,
            json: false,
            short: false,
            show_seed: false,
            date: None,
        };
        assert_eq!(args.get_year(), Local::now().year() as u32);
    }
}
