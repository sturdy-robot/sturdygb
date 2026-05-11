#[cfg(target_arch = "wasm32")]
use super::state::{LoadedGameState, StatusUpdate};
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use rexie::{ObjectStore, Rexie, TransactionMode};

#[cfg(target_arch = "wasm32")]
const SAVE_DATABASE_NAME: &str = "sturdygb";
#[cfg(target_arch = "wasm32")]
const SAVE_DATABASE_VERSION: u32 = 1;
#[cfg(target_arch = "wasm32")]
const SAVE_STORE_NAME: &str = "battery_saves";

#[cfg(target_arch = "wasm32")]
pub(super) struct BatteryRamLoadOutcome {
    pub(super) ram: Option<Vec<u8>>,
    pub(super) status_update: Option<StatusUpdate>,
}

#[cfg(target_arch = "wasm32")]
impl super::EmuApp {
    pub(super) fn import_save_bytes(&mut self, bytes: Vec<u8>) -> Result<StatusUpdate, String> {
        let Some(state) = self.runtime.loaded_game.as_mut() else {
            return Err("Load a ROM before importing a .sav file".to_string());
        };

        state.gb.set_battery_ram(&bytes);
        persist_loaded_game(state);
        self.runtime.error_msg = None;
        Ok(StatusUpdate::success("Imported the selected .sav file."))
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn persist_loaded_game(state: &LoadedGameState) {
    let Some(ram) = state.gb.get_battery_ram() else {
        return;
    };

    let save_key = storage_key(&state.rom_bytes);
    let save_bytes = ram.to_vec();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = persist_battery_ram(save_key, save_bytes).await {
            log::warn!("failed to persist wasm save: {error}");
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn load_battery_ram(rom_bytes: &[u8]) -> BatteryRamLoadOutcome {
    let save_key = storage_key(rom_bytes);
    match load_indexed_db_battery_ram(&save_key).await {
        Ok(Some(ram)) => BatteryRamLoadOutcome {
            ram: Some(ram),
            status_update: None,
        },
        Ok(None) => migrate_legacy_local_storage(rom_bytes).await,
        Err(error) => {
            log::warn!("failed to load wasm save from IndexedDB: {error}");
            migrate_legacy_local_storage(rom_bytes).await
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn persist_battery_ram(save_key: String, save_bytes: Vec<u8>) -> Result<(), String> {
    let database = open_save_database().await?;
    let transaction = database
        .transaction(&[SAVE_STORE_NAME], TransactionMode::ReadWrite)
        .map_err(|error| format!("failed to open save transaction: {error:?}"))?;
    let store = transaction
        .store(SAVE_STORE_NAME)
        .map_err(|error| format!("failed to open save store: {error:?}"))?;
    let value = js_sys::Uint8Array::from(save_bytes.as_slice());
    let key = JsValue::from_str(&save_key);

    store
        .put(&value.into(), Some(&key))
        .await
        .map_err(|error| format!("failed to write save data: {error:?}"))?;
    transaction
        .done()
        .await
        .map_err(|error| format!("save transaction did not complete: {error:?}"))?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn export_loaded_save(loaded_game: &Option<LoadedGameState>) -> Result<(), String> {
    let state = loaded_game
        .as_ref()
        .ok_or_else(|| "Load a ROM before exporting a .sav file".to_string())?;
    let ram = state
        .gb
        .get_battery_ram()
        .ok_or_else(|| "This cartridge does not expose battery-backed save RAM".to_string())?;

    download_save_file(save_file_name(&state.title), ram)
}

#[cfg(target_arch = "wasm32")]
async fn load_indexed_db_battery_ram(save_key: &str) -> Result<Option<Vec<u8>>, String> {
    let database = open_save_database().await?;
    let transaction = database
        .transaction(&[SAVE_STORE_NAME], TransactionMode::ReadOnly)
        .map_err(|error| format!("failed to open save read transaction: {error:?}"))?;
    let store = transaction
        .store(SAVE_STORE_NAME)
        .map_err(|error| format!("failed to open save store: {error:?}"))?;
    let key = JsValue::from_str(save_key);
    let value = store
        .get(key)
        .await
        .map_err(|error| format!("failed to read save data: {error:?}"))?;
    transaction
        .done()
        .await
        .map_err(|error| format!("save read transaction did not complete: {error:?}"))?;

    Ok(value.map(|value| js_sys::Uint8Array::new(&value).to_vec()))
}

#[cfg(target_arch = "wasm32")]
async fn open_save_database() -> Result<Rexie, String> {
    Rexie::builder(SAVE_DATABASE_NAME)
        .version(SAVE_DATABASE_VERSION)
        .add_object_store(ObjectStore::new(SAVE_STORE_NAME))
        .build()
        .await
        .map_err(|error| format!("failed to open IndexedDB database: {error:?}"))
}

#[cfg(target_arch = "wasm32")]
fn browser_local_storage() -> Option<eframe::web_sys::Storage> {
    eframe::web_sys::window()?.local_storage().ok()?
}

#[cfg(target_arch = "wasm32")]
async fn migrate_legacy_local_storage(rom_bytes: &[u8]) -> BatteryRamLoadOutcome {
    let Some(storage) = browser_local_storage() else {
        return BatteryRamLoadOutcome {
            ram: None,
            status_update: None,
        };
    };

    let save_key = storage_key(rom_bytes);
    let Some(encoded) = storage.get_item(&save_key).ok().flatten() else {
        return BatteryRamLoadOutcome {
            ram: None,
            status_update: None,
        };
    };

    let Some(ram) = decode_hex(&encoded) else {
        return BatteryRamLoadOutcome {
            ram: None,
            status_update: Some(StatusUpdate::error(
                "Found a legacy browser save, but it could not be decoded.",
            )),
        };
    };

    match persist_battery_ram(save_key.clone(), ram.clone()).await {
        Ok(()) => {
            let _ = storage.remove_item(&save_key);
            BatteryRamLoadOutcome {
                ram: Some(ram),
                status_update: Some(StatusUpdate::success(
                    "Migrated a legacy browser save into IndexedDB.",
                )),
            }
        }
        Err(error) => {
            log::warn!("failed to migrate legacy wasm save: {error}");
            BatteryRamLoadOutcome {
                ram: Some(ram),
                status_update: Some(StatusUpdate::error(
                    "Found a legacy browser save, but IndexedDB migration failed.",
                )),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn download_save_file(file_name: String, ram: &[u8]) -> Result<(), String> {
    let window = eframe::web_sys::window().ok_or_else(|| "window was not available".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "document was not available".to_string())?;
    let anchor = document
        .create_element("a")
        .map_err(|_| "failed to create download link".to_string())?
        .dyn_into::<eframe::web_sys::HtmlAnchorElement>()
        .map_err(|_| "failed to create HTML anchor element".to_string())?;

    let byte_array = js_sys::Uint8Array::from(ram);
    let parts = js_sys::Array::new();
    parts.push(&byte_array.into());
    let blob = eframe::web_sys::Blob::new_with_u8_array_sequence(&parts)
        .map_err(|_| "failed to create blob for save export".to_string())?;
    let object_url = eframe::web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|_| "failed to create object URL for save export".to_string())?;

    anchor.set_href(&object_url);
    anchor.set_download(&file_name);
    anchor.click();

    let _ = eframe::web_sys::Url::revoke_object_url(&object_url);
    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn save_file_name(title: &str) -> String {
    let mut sanitized = String::with_capacity(title.len());
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            sanitized.push(ch);
        } else if ch.is_whitespace() {
            sanitized.push('_');
        }
    }

    if sanitized.is_empty() {
        sanitized.push_str("sturdygb-save");
    }

    sanitized.push_str(".sav");
    sanitized
}

#[cfg(any(target_arch = "wasm32", test))]
fn storage_key(rom_bytes: &[u8]) -> String {
    format!("sturdygb_sram_{:016x}", fnv1a64(rom_bytes))
}

#[cfg(any(target_arch = "wasm32", test))]
fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET_BASIS;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

#[cfg(test)]
fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[cfg(any(target_arch = "wasm32", test))]
fn decode_hex(encoded: &str) -> Option<Vec<u8>> {
    let bytes = encoded.as_bytes();
    if bytes.len() % 2 != 0 {
        return None;
    }

    let mut decoded = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let high = decode_hex_nibble(chunk[0])?;
        let low = decode_hex_nibble(chunk[1])?;
        decoded.push((high << 4) | low);
    }
    Some(decoded)
}

#[cfg(any(target_arch = "wasm32", test))]
fn decode_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_hex, encode_hex, save_file_name, storage_key};

    #[test]
    fn hex_round_trips_bytes() {
        let bytes = [0x00, 0x7f, 0x80, 0xfe, 0xff];
        assert_eq!(decode_hex(&encode_hex(&bytes)), Some(bytes.to_vec()));
    }

    #[test]
    fn storage_key_changes_with_rom_bytes() {
        let left = storage_key(&[0x01, 0x02, 0x03]);
        let right = storage_key(&[0x01, 0x02, 0x04]);
        assert_ne!(left, right);
    }

    #[test]
    fn save_file_name_sanitizes_title() {
        assert_eq!(save_file_name("Pokemon Crystal"), "Pokemon_Crystal.sav");
        assert_eq!(save_file_name("PM_CRYSTALBYTEA"), "PM_CRYSTALBYTEA.sav");
    }
}
