use std::collections::HashMap;

use std::sync::{
    Arc,
    Mutex,
    MutexGuard
};

use crate::config;

pub mod integrations;

use integrations::Game;

static mut GAMES_SINGLETON: Option<HashMap<String, Arc<Game>>> = None;

pub fn init() -> anyhow::Result<()> {
    let integration_scripts = config::get().games.integrations.path;

    let mut games = HashMap::new();

    for entry in integration_scripts.read_dir()?.flatten() {
        if entry.path().is_dir() {
            let game = Game::new(entry.path().join("manifest.json"))?;

            games.insert(entry.file_name().to_string_lossy().to_string(), Arc::new(game));
        }
    }

    unsafe {
        GAMES_SINGLETON = Some(games);
    }

    Ok(())
}

pub fn get<'a>(name: impl AsRef<str>) -> anyhow::Result<Option<&'a Arc<Game>>> {
    unsafe {
        let Some(singleton) = &mut GAMES_SINGLETON else {
            init()?;

            return get(name);
        };

        if let Some(result) = singleton.get(name.as_ref()) {
            return Ok(Some(result));
        }

        Ok(None)
    }
}

/// # Safety
/// 
/// This function is called by the game cards which are generated from the `games::list()` method,
/// so every `get_unsafe()` call will contain an actual game's name
pub unsafe fn get_unsafe<'a>(name: impl AsRef<str>) -> &'a Arc<Game> {
    GAMES_SINGLETON.as_ref()
        .unwrap_unchecked()
        .get(name.as_ref())
        .unwrap_unchecked()
}

pub fn list<'a>() -> anyhow::Result<&'a HashMap<String, Arc<Game>>> {
    unsafe {
        match &GAMES_SINGLETON {
            Some(singleton) => Ok(singleton),
            None => {
                init()?;

                list()
            }
        }
    }
}
