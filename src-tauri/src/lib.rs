use std::{path::PathBuf, str::FromStr};

use jinhsi_core::gacha::{
    local::LocalGachaSource, url::UrlGachaSource, GachaError, GachaLog, GachaService,
};
use url::Url;

#[tauri::command]
async fn get_gachalog_from_url(url: String) -> Result<Vec<GachaLog>, String> {
    let source = UrlGachaSource::new(
        Url::parse(&url).map_err(|_| GachaError::InvalidUrl { url }.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    return source.get_gacha_data().await.map_err(|e| format!("{}", e));
}

#[tauri::command]
async fn get_gachalog_from_local(path: Option<String>) -> Result<Vec<GachaLog>, String> {
    let path = {
        if let Some(s) = &path {
            if let Ok(pb) = PathBuf::from_str(s) {
                Some(pb)
            } else {
                log::error!("invalid path {:?}", path);
                None
            }
        } else {
            None
        }
    };
    let source = LocalGachaSource::new(path);
    return source.get_gacha_data().await.map_err(|e| format!("{}", e));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_gachalog_from_url,
            get_gachalog_from_local
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
