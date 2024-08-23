pub(super) mod gacha;
use std::error::Error;

pub(super) fn handle_error<T: Error>(error: T) -> String {
    log::error!("cmd error: {}", error);
    error.to_string()
}
