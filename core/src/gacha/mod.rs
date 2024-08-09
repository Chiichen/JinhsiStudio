pub mod local;
pub mod url;

use std::error::Error;

use ::url::Url;
use local::LocalGachaSource;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use snafu::Snafu;
use url::UrlGachaSource;

#[derive(Debug, Snafu)]
pub enum GachaError {
    #[snafu(display("Invalid Url: {} ", url))]
    InvalidUrl { url: String },
    #[snafu(display("Unable to probe gacha url, input it manually"))]
    ProbeFailed,
    #[snafu(display("Unable to fetch gacha from url {} since {}", url, source))]
    RequestFailed { url: Url, source: Box<dyn Error> },
}

pub enum GachaLogSource {
    Url(UrlGachaSource),
    Local(LocalGachaSource), //Probe the client cache locally
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GachaLogItem {
    #[serde(alias = "resourceId")]
    id: usize,
    #[serde(alias = "qualityLevel")]
    rarity: usize,
    name: String,
    #[serde(alias = "time")]
    date: String,
}
/// Convene enum which is expected to match the Wuthering Waves' official backend server protocol
///
///
/// ## Tips
///
/// - We use `Serialize_repr, Deserialize_repr` to ensure this enum to be serialized as number in ipc channel, otherwise, it will be serialized as plain string such as `"EventCharater"``.
///
#[derive(Debug, Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GachaLog {
    convene: Convene,
    items: Vec<GachaLogItem>,
}

pub trait GachaService {
    async fn get_gacha_data(&self) -> Result<Vec<GachaLog>, GachaError>;
}

impl GachaService for GachaLogSource {
    async fn get_gacha_data(&self) -> Result<Vec<GachaLog>, GachaError> {
        match self {
            GachaLogSource::Url(source) => source.get_gacha_data().await,
            GachaLogSource::Local(source) => source.get_gacha_data().await,
        }
    }
}
