use std::path::PathBuf;

use crate::games;

use crate::games::integrations::standards::addons::{
    Addon,
    AddonsGroup
};

use crate::ui::components::game_card::CardInfo;
use crate::ui::components::tasks_queue::delete_files_task::DeleteFilesQueuedTask;

use super::MainAppMsg;

type HeapResult<T> = Result<T, Box<MainAppMsg>>;

#[inline]
pub async fn get_uninstall_addon_task(game_info: &CardInfo, addon: &Addon, group: &AddonsGroup) -> HeapResult<Box<DeleteFilesQueuedTask>> {
    let addon_path = addon.get_installation_path(&group.name, game_info.get_name(), game_info.get_edition()).await
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to find {} addon installation path", game_info.get_title()),
            message: Some(err.to_string())
        }))?;

    let game = unsafe {
        games::get_unsafe(game_info.get_name())
    };

    let paths = game.get_addon_paths(&group.name, &addon.name, addon_path.to_string_lossy(), game_info.get_edition()).await
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to find {} addon installation path", game_info.get_title()),
            message: Some(err.to_string())
        }))?
        .into_iter()
        .map(PathBuf::from)
        .collect();

    Ok(Box::new(DeleteFilesQueuedTask {
        paths
    }))
}
