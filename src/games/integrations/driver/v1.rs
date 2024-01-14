use mlua::prelude::*;

use super::*;

#[derive(Debug)]
pub struct GameDriver {
    lua: Lua
}

impl GameDriverExt for GameDriver {
    type Error = anyhow::Error;

    #[inline]
    fn get_editions_list(&self) -> Result<Vec<GameEdition>, Self::Error> {
        let editions = self.lua.globals()
            .call_function::<_, LuaTable>("v1_game_get_editions_list", ())?
            .sequence_values::<LuaTable>()
            .flatten()
            .map(|edition| GameEdition::from_table(edition, IntegrationStandard::V1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(editions)
    }

    #[inline]
    fn get_card_picture(&self, edition: &str) -> Result<String, Self::Error> {
        Ok(self.lua.globals().call_function("v1_visual_get_card_picture", edition)?)
    }

    #[inline]
    fn get_background_picture(&self, edition: &str) -> Result<String, Self::Error> {
        Ok(self.lua.globals().call_function("v1_visual_get_background_picture", edition)?)
    }

    #[inline]
    fn get_details_style(&self, edition: &str) -> Result<Option<String>, Self::Error> {
        if !self.lua.globals().contains_key("v1_visual_get_details_background_css")? {
            return Ok(None);
        }

        Ok(self.lua.globals().call_function("v1_visual_get_details_background_css", edition)?)
    }

    #[inline]
    fn is_installed(&self, path: &str, edition: &str) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_is_installed", (
            path,
            edition
        ))?)
    }

    #[inline]
    fn get_version(&self, path: &str, edition: &str) -> Result<Option<String>, Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_get_version", (
            path,
            edition
        ))?)
    }

    #[inline]
    fn get_download(&self, edition: &str) -> Result<Download, Self::Error> {
        let download = self.lua.globals().call_function("v1_game_get_download", edition)?;

        Download::from_table(download, IntegrationStandard::V1)
    }

    #[inline]
    fn get_diff(&self, path: &str, edition: &str) -> Result<Option<Diff>, Self::Error> {
        let diff = self.lua.globals().call_function("v1_game_get_diff", (
            path,
            edition
        ))?;

        match diff {
            Some(diff) => Ok(Some(Diff::from_table(diff, IntegrationStandard::V1)?)),
            None => Ok(None)
        }
    }

    #[inline]
    fn get_status(&self, path: &str, edition: &str) -> Result<Option<GameStatus>, Self::Error> {
        let status = self.lua.globals().call_function("v1_game_get_status", (
            path,
            edition
        ))?;

        match status {
            Some(status) => Ok(Some(GameStatus::from_table(status,IntegrationStandard::V1)?)),
            None => Ok(None)
        }
    }

    #[inline]
    fn get_launch_options(&self, game_path: &str, addons_path: &str, edition: &str) -> Result<GameLaunchOptions, Self::Error> {
        let options = self.lua.globals().call_function("v1_game_get_launch_options", (
                game_path,
                addons_path,
                edition
            ))?;

        GameLaunchOptions::from_table(options, IntegrationStandard::V1)
    }

    #[inline]
    fn is_process_running(&self, game_path: &str, edition: &str) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_is_running", (
            game_path,
            edition
        ))?)
    }

    #[inline]
    fn kill_process(&self, game_path: &str, edition: &str) -> Result<(), Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_kill", (
            game_path,
            edition
        ))?)
    }

    #[inline]
    fn get_integrity(&self, game_path: &str, edition: &str) -> Result<Vec<IntegrityInfo>, Self::Error> {
        let integrity = self.lua.globals()
            .call_function::<_, LuaTable>("v1_game_get_integrity_info", (
                game_path,
                edition
            ))?
            .sequence_values::<LuaTable>()
            .flatten()
            .map(|info| IntegrityInfo::from_table(info, IntegrationStandard::V1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(integrity)
    }

    #[inline]
    fn has_diff_transition(&self) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().contains_key("v1_game_diff_transition")?)
    }

    #[inline]
    fn run_diff_transition(&self, transition_path: &str, edition: &str) -> Result<(), Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_diff_transition", (
            transition_path,
            edition
        ))?)
    }

    #[inline]
    fn has_diff_post_transition(&self) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().contains_key("v1_game_diff_post_transition")?)
    }

    #[inline]
    fn run_diff_post_transition(&self, game_path: &str, edition: &str) -> Result<(), Self::Error> {
        Ok(self.lua.globals().call_function("v1_game_diff_post_transition", (
            game_path,
            edition
        ))?)
    }

    #[inline]
    fn has_integrity_hash(&self) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().contains_key("v1_integrity_hash")?)
    }

    #[inline]
    fn integrity_hash(&self, algorithm: &str, data: &[u8]) -> Result<String, Self::Error> {
        Ok(self.lua.globals().call_function("v1_integrity_hash", (
            algorithm,
            self.lua.create_string(data)?
        ))?)
    }
}

impl AddonsDriverExt for GameDriver {
    type Error = anyhow::Error;

    #[inline]
    fn get_list(&self, edition: &str) -> Result<Vec<AddonsGroup>, Self::Error> {
        let addons = self.lua.globals()
            .call_function::<_, LuaTable>("v1_addons_get_list", edition)?
            .sequence_values::<LuaTable>()
            .flatten()
            .map(|group| AddonsGroup::from_table(group, IntegrationStandard::V1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(addons)
    }

    #[inline]
    fn is_installed(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().call_function("v1_addons_is_installed", (
            group_name,
            addon_name,
            addon_path,
            edition
        ))?)
    }

    #[inline]
    fn get_version(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Option<String>, Self::Error> {
        Ok(self.lua.globals().call_function("v1_addons_get_version", (
            group_name,
            addon_name,
            addon_path,
            edition
        ))?)
    }

    #[inline]
    fn get_download(&self, group_name: &str, addon_name: &str, edition: &str) -> Result<Download, Self::Error> {
        let download = self.lua.globals().call_function("v1_addons_get_download", (
            group_name,
            addon_name,
            edition
        ))?;

        Download::from_table(download, IntegrationStandard::V1)
    }

    #[inline]
    fn get_diff(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Option<Diff>, Self::Error> {
        let diff = self.lua.globals().call_function("v1_addons_get_diff", (
            group_name,
            addon_name,
            addon_path,
            edition
        ))?;

        match diff {
            Some(diff) => Ok(Some(Diff::from_table(diff, IntegrationStandard::V1)?)),
            None => Ok(None)
        }
    }

    #[inline]
    fn get_paths(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Vec<String>, Self::Error> {
        Ok(self.lua.globals().call_function("v1_addons_get_paths", (
            group_name,
            addon_name,
            addon_path,
            edition
        ))?)
    }

    #[inline]
    fn get_integrity(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Vec<IntegrityInfo>, Self::Error> {
        let info = self.lua.globals()
            .call_function::<_, LuaTable>("v1_addons_get_integrity_info", (
                group_name,
                addon_name,
                addon_path,
                edition
            ))?
            .sequence_values::<LuaTable>()
            .flatten()
            .map(|info| IntegrityInfo::from_table(info, IntegrationStandard::V1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(info)
    }

    #[inline]
    fn has_diff_transition(&self) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().contains_key("v1_addons_diff_transition")?)
    }

    #[inline]
    fn run_diff_transition(&self, group_name: &str, addon_name: &str, transition_path: &str, edition: &str) -> Result<(), Self::Error> {
        Ok(self.lua.globals().call_function("v1_addons_diff_transition", (
            group_name,
            addon_name,
            transition_path,
            edition
        ))?)
    }

    #[inline]
    fn has_diff_post_transition(&self) -> Result<bool, Self::Error> {
        Ok(self.lua.globals().contains_key("v1_addons_diff_post_transition")?)
    }

    #[inline]
    fn run_diff_post_transition(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<(), Self::Error> {
        Ok(self.lua.globals().call_function("v1_addons_diff_post_transition", (
            group_name,
            addon_name,
            addon_path,
            edition
        ))?)
    }
}
