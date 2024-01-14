use std::path::{Path, PathBuf};

use serde_json::Value as Json;
use mlua::prelude::*;

pub mod manifest;
pub mod standards;
pub mod driver;

use manifest::Manifest;
use driver::prelude::*;
use standards::prelude::*;

pub struct Game {
    pub manifest: Manifest,
    pub driver: Box<dyn GeneralDriver<anyhow::Error>>
}

// Let (at least for now) lua scripts maintainers resolve
// possible data races themselves
// 
// FIXME: use Mutex or RwLock, or anything else but please remove this shit

unsafe impl Send for Game {}
unsafe impl Sync for Game {}

impl Game {
    pub fn new(manifest_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let manifest = std::fs::read(manifest_path.as_ref())?;
        let manifest = serde_json::from_slice::<Json>(&manifest)?;
        let manifest = Manifest::from_json(&manifest)?;

        let script_path = PathBuf::from(&manifest.script_path);

        let script_path = if script_path.is_absolute() {
            script_path
        } else {
            manifest_path.as_ref()
                .parent()
                .map(|path| path.join(&script_path))
                .unwrap_or(script_path)
        };

        let script = std::fs::read_to_string(&script_path)?;

        let game = Self {
            manifest,
            driver: match manifest.script_standard {
                IntegrationStandard::V1 => {
                    Box::new(GameDriverV1 {
                        lua: Lua::new()
                    })
                }
            }
        };

        game.lua.globals().set("v1_network_http_get", game.lua.create_function(|lua, uri: String| {
            anime_game_core::network::minreq::get(uri)
                .send()
                .map(|result| lua.create_string(result.as_bytes()))
                .map_err(LuaError::external)
        })?)?;

        game.lua.globals().set("v1_json_decode", game.lua.create_function(|lua, json: String| {
            serde_json::from_str::<Json>(&json)
                .map(|value| lua.to_value(&value))
                .map_err(LuaError::external)
        })?)?;

        game.lua.load(script).exec()?;

        Ok(game)
    }
}
