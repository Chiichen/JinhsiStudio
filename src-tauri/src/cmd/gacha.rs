use std::{path::PathBuf, str::FromStr};

use jinhsi_core::gacha::{local::LocalGachaSource, url::UrlGachaSource, GachaLog, GachaService};

use super::handle_error;

#[tauri::command]
pub(crate) async fn get_gachalog_from_url(url: String) -> Result<Vec<GachaLog>, String> {
    let source = UrlGachaSource::new(url).map_err(handle_error)?;
    source.get_gacha_data().await.map_err(handle_error)
}

#[tauri::command]
pub(crate) async fn get_gachalog_from_local(path: Option<String>) -> Result<Vec<GachaLog>, String> {
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
    source.get_gacha_data().await.map_err(handle_error)
}

#[tauri::command]
pub(crate) async fn update_gachalog_from_url(
    data: Option<Vec<GachaLog>>,
    url: String,
) -> Result<Vec<GachaLog>, String> {
    let source = UrlGachaSource::new(url).map_err(handle_error)?;
    let new_data = source.get_gacha_data().await.map_err(handle_error)?;
    if let Some(data) = data {
        if data.is_empty() {
            return Ok(new_data);
        }
        let merged_data = data
            .iter()
            .map(|old_log| {
                for new_log in &new_data {
                    if let Some(merged_log) = old_log.merge(new_log) {
                        return merged_log;
                    }
                }
                unreachable!("Incompatible convene type");
                // return old_log.clone();
            })
            .collect();
        Ok(merged_data)
    } else {
        Ok(new_data)
    }
}

#[tauri::command]
pub(crate) async fn update_gachalog_from_local(
    data: Option<Vec<GachaLog>>,
    path: Option<String>,
) -> Result<Vec<GachaLog>, String> {
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
    let new_data = source.get_gacha_data().await.map_err(handle_error)?;
    if let Some(data) = data {
        if data.is_empty() {
            return Ok(new_data);
        }
        let merged_data = data
            .iter()
            .map(|old_log| {
                for new_log in &new_data {
                    if let Some(merged_log) = old_log.merge(new_log) {
                        return merged_log;
                    }
                }
                unreachable!("Incompatible convene type");
                // return old_log.clone();
            })
            .collect();
        Ok(merged_data)
    } else {
        Ok(new_data)
    }
}
