use crate::config;
use crate::config::games::settings::GameSettings;

use crate::games;
use crate::games::integrations::Game;
use crate::games::integrations::standards::game::Edition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameListEntry {
    pub game_name: String,
    pub game_title: String,
    pub game_developer: String,
    pub edition: Edition,
    pub card_picture: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GamesList {
    pub installed: Vec<GameListEntry>,
    pub available: Vec<GameListEntry>
}

#[inline]
pub fn init_games() -> anyhow::Result<()> {
    games::init()
}

#[inline]
async fn get_game_entries(game: &Game, settings: GameSettings) -> anyhow::Result<Vec<(bool, GameListEntry)>> {
    let mut results = Vec::new();

    for edition in game.get_game_editions_list().await? {
        let entry = GameListEntry {
            game_name: game.manifest.game_name.clone(),
            game_title: game.manifest.game_title.clone(),
            game_developer: game.manifest.game_developer.clone(),
            card_picture: game.get_card_picture(&edition.name).await?,
            edition
        };

        let installed = game.is_game_installed(settings.paths[&entry.edition.name].game.to_string_lossy()).await?;

        results.push((installed, entry));
    }

    Ok(results)
}

#[inline]
pub async fn register_games_styles() -> anyhow::Result<()> {
    let mut styles = String::new();

    for (name, game) in games::list()?.iter() {
        for edition in game.get_game_editions_list().await? {
            if let Some(style) = game.get_details_background_style(&edition.name).await? {
                styles = format!("{styles} .game-details--{name}--{} {{ {style} }}", edition.name);
            }
        }
    }

    gtk::glib::MainContext::default().spawn(async move {
        relm4::set_global_css(&styles);
    });

    Ok(())
}

#[inline]
pub async fn get_games_list() -> anyhow::Result<GamesList> {
    let settings = config::get().games;

    let games = games::list()?;

    let mut installed = Vec::new();
    let mut available = Vec::with_capacity(games.len());

    for game in games.values() {
        let settings = settings.get_game_settings(game).await?;

        let entries = get_game_entries(game, settings).await?;

        let installed_entries = entries.iter()
            .filter_map(|(installed, entry)| installed.then_some(entry))
            .cloned();

        let available_entries = entries.iter()
            .filter_map(|(installed, entry)| (!installed).then_some(entry))
            .cloned();

        installed.extend(installed_entries);
        available.extend(available_entries);
    }

    Ok(GamesList {
        installed,
        available
    })
}
