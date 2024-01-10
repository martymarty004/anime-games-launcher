use std::sync::Arc;

use rusty_pool::{
    ThreadPool,
    JoinHandle
};

use crate::config;
use crate::config::games::prelude::GameSettings;
use crate::config::games::settings::edition_addons::GameEditionAddon;

use crate::games;
use crate::games::integrations::Game;

use crate::games::integrations::standards::diff::{
    Diff,
    DiffStatus
};

use crate::games::integrations::standards::addons::{
    Addon,
    AddonsGroup
};

use crate::games::integrations::standards::game::Edition;
use crate::ui::components::game_card::CardInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddonsListEntry {
    pub game_info: CardInfo,
    pub addon: Addon,
    pub group: AddonsGroup
}

#[inline]
pub fn is_addon_enabled(enabled_addons: &[GameEditionAddon], addon: &Addon, group: &AddonsGroup) -> bool {
    addon.required || enabled_addons.iter().any(|enabled_addon| {
        enabled_addon.group == group.name && enabled_addon.name == addon.name
    })
}

#[inline]
fn check_addon(
    game: &Game,
    game_info: &CardInfo,
    enabled_addons: &[GameEditionAddon],
    addon: &Addon,
    group: &AddonsGroup
) -> anyhow::Result<Option<AddonsListEntry>> {
    if is_addon_enabled(enabled_addons, addon, group) {
        let addon_path = addon.get_installation_path(&group.name, &game.manifest.game_name, game_info.get_edition())?;

        let installed = game.is_addon_installed(
            &group.name,
            &addon.name,
            &addon_path.to_string_lossy(),
            game_info.get_edition()
        )?;

        let entry = AddonsListEntry {
            game_info: game_info.clone(),
            addon: addon.clone(),
            group: group.clone()
        };

        if !installed {
            return Ok(Some(entry));
        }

        let diff = game.get_addon_diff(
            &group.name,
            &addon.name,
            &addon_path.to_string_lossy(),
            game_info.get_edition()
        )?;

        // TODO: handle "unavailable" status
        if let Some(Diff { status: DiffStatus::Outdated, .. }) = diff {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

struct GameAddons {
    pub edition: Edition,
    pub addons: Vec<AddonsGroup>
}

#[inline]
fn get_edition_addons_pool(pool: &ThreadPool, game: &Arc<Game>) -> anyhow::Result<Vec<JoinHandle<anyhow::Result<GameAddons>>>> {
    let mut tasks = Vec::new();

    println!("Building {} editions addons pool", game.manifest.game_title);

    for edition in game.get_game_editions_list()? {
        let game = game.clone();

        println!("  - Populating with the {} edition", edition.title);

        tasks.push(pool.evaluate(move || -> anyhow::Result<GameAddons> {
            println!("> call get_addons_list");
            game.get_addons_list(&edition.name)
                .map(|addons| GameAddons {
                    edition,
                    addons
                })
        }));
    }

    println!("Built {} editions addons pool", game.manifest.game_title);

    Ok(tasks)
}

#[inline]
fn get_check_addons_pool(pool: &ThreadPool, game: &Arc<Game>, game_settings: &GameSettings) -> anyhow::Result<Vec<JoinHandle<anyhow::Result<Option<AddonsListEntry>>>>> {
    let mut tasks = Vec::new();

    println!("Building {} addons checking pool", game.manifest.game_title);

    for task in get_edition_addons_pool(pool, game)? {
        let result = task.await_complete()?;

        let enabled_addons = game_settings.addons[&result.edition.name].clone();

        let game_info = CardInfo::Game {
            name: game.manifest.game_name.clone(),
            title: game.manifest.game_title.clone(),
            developer: game.manifest.game_developer.clone(),
            picture_uri: game.get_card_picture(&result.edition.name)?,
            edition: result.edition.name.clone()
        };

        println!("  - Populating with the {} edition", result.edition.title);

        for group in result.addons {
            for addon in &group.addons {
                let game = game.clone();
                let game_info = game_info.clone();
                let enabled_addons = enabled_addons.clone();
                let addon = addon.clone();
                let group = group.clone();

                println!("    - Populating with the {} addon from {} group", addon.title, group.title);

                tasks.push(pool.evaluate(move || {
                    check_addon(&game, &game_info, &enabled_addons, &addon, &group)
                }));
            }
        }
    }

    println!("Built {} addons checking pool", game.manifest.game_title);

    Ok(tasks)
}

#[inline]
pub fn check_addons() -> anyhow::Result<Vec<AddonsListEntry>> {
    let config = config::get();

    let pool = rusty_pool::Builder::new()
        .name(String::from("check_addons"))
        .core_size(8)
        .build();

    let mut addons = Vec::new();

    for game in games::list()?.values() {
        let settings = config.games.get_game_settings(game)?;

        for task in get_check_addons_pool(&pool, game, &settings)? {
            if let Some(addon) = task.await_complete()? {
                addons.push(addon);
            }
        }
    }

    Ok(addons)
}
