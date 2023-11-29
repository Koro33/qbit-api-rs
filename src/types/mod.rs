pub mod app;
pub mod auth;
pub mod log;
pub mod search;
pub mod sync;
pub mod torrents;
pub mod transfer;

/// ### NOTE:
/// this custom serializer module is written to solve the problem,
/// that the serde_urlencoded cannot deserialize the nested struct,
/// which is the default dependency of the reqwest
mod preferences_serialize {

    use serde::{self, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, s: S) -> Result<S::Ok, S::Error>
    where
        T: ?Sized + Serialize,
        S: Serializer,
    {
        match serde_json::to_string(value) {
            Ok(json) => s.serialize_str(&json),
            Err(_) => Err(serde::ser::Error::custom("Failed to serialize &T to json")),
        }
    }
}

/// serialize & deserialize between `Vec<String>` and String with vertical bar `|`
///
/// e.g. `"a|b|c" <=> vec!["a", "b", "c"]`
pub mod string_saperated_with_vertical_bar {

    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join("|");
        s.serialize_str(&string_line)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_line = String::deserialize(deserializer)?;
        let string_items: Vec<String> = string_line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        Ok(string_items)
    }
}

/// module to serialize & deserialize between `Vec<String>` and String with `\n`
///
/// e.g. `"a\nb\nc" <=> vec!["a", "b", "c"]`
mod string_saperated_with_backslash_n {

    use serde::Serializer;

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join("\n");
        s.serialize_str(&string_line)
    }
}

/// module to serialize & deserialize between `Vec<String>` and String with `,`
///
/// e.g. `"a,b,c" <=> vec!["a", "b", "c"]`
mod string_saperated_with_comma {

    use serde::Serializer;

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join(",");
        s.serialize_str(&string_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    fn read_json_file<P: AsRef<Path>>(path: P) -> String {
        let mut file = BufReader::new(File::open(path).unwrap());
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_deserialize_preferences_response() {
        let s = read_json_file("./tests/PreferencesResponse.json");
        let _p: app::Preferences = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_deserialize_maindata_response() {
        let s = read_json_file("./tests/MaindataResponse.json");
        let _p: sync::MaindataResponse = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_serialize_deserialize_hashes() {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct TestHash {
            #[serde(with = "string_saperated_with_vertical_bar")]
            hashes: Vec<String>,
        }

        let serialized = r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6|a585051959d4e06e71da2f4306547a08348e5d34"}"#;

        let deserialized = TestHash {
            hashes: vec![
                "7e2fc0391f2d855affed3b0545927bddd5189bc6".to_string(),
                "a585051959d4e06e71da2f4306547a08348e5d34".to_string(),
            ],
        };
        let de: TestHash = serde_json::from_str(serialized).unwrap();
        assert_eq!(de, deserialized);

        let se = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(se, serialized);
    }

    #[test]
    fn test_set_share_limits_form() {
        let limit = torrents::RatioLimit::Limit { ratio_limit: 5.5 };
        let mut limit_form = torrents::SetShareLimitsForm {
            hashes: vec!["7e2fc0391f2d855affed3b0545927bddd5189bc6".to_string()],
            ratio_limit: limit,
            seeding_time_limit: 0,
        };

        let s = serde_json::to_string(&limit_form).unwrap();
        assert_eq!(
            s,
            r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6","ratioLimit":5.5,"seedingTimeLimit":0}"#
        );

        let special = torrents::RatioLimit::Special { ratio_limit: -1 };
        limit_form.ratio_limit = special;
        let s = serde_json::to_string(&limit_form).unwrap();
        assert_eq!(
            s,
            r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6","ratioLimit":-1,"seedingTimeLimit":0}"#
        );
    }
}
