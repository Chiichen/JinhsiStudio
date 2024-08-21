pub mod local;
pub mod url;

use std::error::Error;

use ::url::Url;
use chrono::NaiveDateTime;
use local::LocalGachaSource;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use snafu::Snafu;
use url::UrlGachaSource;

#[derive(Debug, Snafu)]
pub enum GachaError {
    #[snafu(display("Invalid Url: {} ", desc))]
    InvalidUrl { desc: String },
    #[snafu(display("Unable to probe gacha url, input it manually"))]
    ProbeFailed,
    #[snafu(display("Unable to fetch gacha from url {} since {}", url, source))]
    RequestFailed { url: Url, source: Box<dyn Error> },
}

pub enum GachaLogSource {
    Url(UrlGachaSource),
    Local(LocalGachaSource), //Probe the client cache locally
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaLogItem {
    #[serde(alias = "resourceId")]
    id: usize,
    #[serde(alias = "qualityLevel")]
    rarity: usize,
    name: String,
    #[serde(alias = "time", with = "date_format")]
    date: NaiveDateTime,
}

impl PartialEq for GachaLogItem {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

impl PartialOrd for GachaLogItem {
    fn partial_cmp(&self, other: &GachaLogItem) -> std::option::Option<std::cmp::Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl Eq for GachaLogItem {}

impl Ord for GachaLogItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}
/// Convene enum which is expected to match the Wuthering Waves' official backend server protocol
///
///
/// ## Tips
///
/// - We use `Serialize_repr, Deserialize_repr` to ensure this enum to be serialized as number in ipc channel, otherwise, it will be serialized as plain string such as `"EventCharater"``.
///
#[derive(
    Clone, Debug, Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive, PartialEq, Eq,
)]
#[repr(u8)]
pub enum Convene {
    EventCharacter = 1,           //角色活动唤取 Featured Resonator Convene
    EventWeapon = 2,              //武器活动唤取 Featured Weapon Convene
    PermanentCharacter = 3,       // 角色常驻唤取 Standard Resonator Convene
    PermanentWeapon = 4,          //武器常驻唤取 Standard Weapon Convene
    Beginner = 5,                 //新手唤取 Beginner Convene
    BeginnerSelected = 6,         //新手自选唤取 Beginner's Choice Convene
    BeginnerGiveBackSelected = 7, //新手自选唤取(感恩回馈) Beginner's Choice Convene（Giveback Custom Convene)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GachaLog {
    convene: Convene,
    items: Vec<GachaLogItem>,
}

impl GachaLog {
    pub fn new(convene: Convene, items: Vec<GachaLogItem>) -> Self {
        Self { convene, items }
    }
    /// Merge the [`GachaLog::items`] in two `GachaLog` objects.
    ///
    /// Requiring the [`GachaLog::items`] to be sorted in descending time order.
    /// Also, the two GachaLog objects are currently required to be two consecutive subsequences in the complete sequence.
    /// For example, if the complete sequence is \[9,8,8,7,6,5\], then \[9,8,8,7\] and \[8,8,7,6,5\] meet the requirement, but \[9,8,7\] and \[7,6,5\] do not.
    ///
    /// # Return
    /// - `None`: if [`GachaLog::convene`] between `self` and `other` doesn't match.
    /// - `Some(GachaLog)`: The merged `GachaLog`
    pub fn merge(&self, other: &GachaLog) -> Option<GachaLog> {
        if self.convene != other.convene {
            return None;
        }
        if self.items.is_empty() {
            return Some(other.clone());
        }
        if other.items.is_empty() {
            return Some(self.clone());
        }
        let max_self = self.items.first().unwrap();
        let max_other = other.items.first().unwrap();
        let (mut left, right) = if max_self >= max_other {
            (self.clone(), other)
        } else {
            (other.clone(), self)
        };
        let left_last = left.items.last().unwrap();
        let split_point = right.items.partition_point(|x| x >= left_last);
        left.items.extend_from_slice(&right.items[split_point..]);
        Some(left)
    }

    pub fn convene(&self) -> &Convene {
        &self.convene
    }

    pub fn items(&self) -> &[GachaLogItem] {
        &self.items
    }
}

pub trait GachaService {
    fn get_gacha_data(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<GachaLog>, GachaError>> + Send;
}

impl GachaService for GachaLogSource {
    async fn get_gacha_data(&self) -> Result<Vec<GachaLog>, GachaError> {
        match self {
            GachaLogSource::Url(source) => source.get_gacha_data().await,
            GachaLogSource::Local(source) => source.get_gacha_data().await,
        }
    }
}

/// To deserialize DateTime data from request
/// ## Reference
/// [custom-date-format](https://serde.rs/custom-date-format.html)
mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use crate::gacha::date_format;
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TimeStruct {
        #[serde(alias = "time", with = "date_format")]
        date: NaiveDateTime,
    }
    #[test]
    fn test_baic_chrono() {
        let case1 = "2024-01-01 15:35:00";
        let expected_result1: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(15, 35, 00)
            .unwrap();
        let result1 = NaiveDateTime::parse_from_str(&case1, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(expected_result1, result1);
        let case2 = "2024-01-01 15:35:00";
        let result2 = NaiveDateTime::parse_from_str(&case2, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result1, result2);
        let case3 = "2024-01-01 16:00:00";
        let result3 = NaiveDateTime::parse_from_str(&case3, "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(result3 > result1);
        let case4 = "2024-01-01 15:00:00";
        let result4 = NaiveDateTime::parse_from_str(&case4, "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(result4 < result1)
    }

    #[test]
    fn test_deserialize_json_chrono() {
        let case1 = r#"{"time":"2024-01-01 15:35:00"}"#;
        let expected_result1: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(15, 35, 00)
            .unwrap();
        let result1: TimeStruct = serde_json::from_str(case1).unwrap();
        assert_eq!(expected_result1, result1.date)
    }
}
