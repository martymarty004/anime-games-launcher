use crate::games::integrations::standards::prelude::*;

pub mod v1;

pub mod prelude {
    pub use super::GeneralDriver;
    pub use super::GameDriverExt;
    pub use super::AddonsDriverExt;

    pub use super::v1::GameDriver as GameDriverV1;
}

pub trait GeneralDriver<E>: GameDriverExt<Error = E> + AddonsDriverExt<Error = E> {

}

pub trait GameDriverExt {
    type Error;

    fn get_editions_list(&self) -> Result<Vec<GameEdition>, Self::Error>;

    fn get_card_picture(&self, edition: &str) -> Result<String, Self::Error>;
    fn get_background_picture(&self, edition: &str) -> Result<String, Self::Error>;
    fn get_details_style(&self, edition: &str) -> Result<Option<String>, Self::Error>;

    fn is_installed(&self, path: &str, edition: &str) -> Result<bool, Self::Error>;
    fn get_version(&self, path: &str, edition: &str) -> Result<Option<String>, Self::Error>;

    fn get_download(&self, edition: &str) -> Result<Download, Self::Error>;
    fn get_diff(&self, path: &str, edition: &str) -> Result<Option<Diff>, Self::Error>;

    fn get_status(&self, path: &str, edition: &str) -> Result<Option<GameStatus>, Self::Error>;
    fn get_launch_options(&self, game_path: &str, addons_path: &str, edition: &str) -> Result<GameLaunchOptions, Self::Error>;

    fn is_process_running(&self, game_path: &str, edition: &str) -> Result<bool, Self::Error>;
    fn kill_process(&self, game_path: &str, edition: &str) -> Result<(), Self::Error>;

    fn get_integrity(&self, game_path: &str, edition: &str) -> Result<Vec<IntegrityInfo>, Self::Error>;

    fn has_diff_transition(&self) -> Result<bool, Self::Error>;
    fn run_diff_transition(&self, transition_path: &str, edition: &str) -> Result<(), Self::Error>;

    fn has_diff_post_transition(&self) -> Result<bool, Self::Error>;
    fn run_diff_post_transition(&self, game_path: &str, edition: &str) -> Result<(), Self::Error>;

    fn has_integrity_hash(&self) -> Result<bool, Self::Error>;
    fn integrity_hash(&self, algorithm: &str, data: &[u8]) -> Result<String, Self::Error>;
}

pub trait AddonsDriverExt {
    type Error;

    fn get_list(&self, edition: &str) -> Result<Vec<AddonsGroup>, Self::Error>;

    fn is_installed(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<bool, Self::Error>;
    fn get_version(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Option<String>, Self::Error>;

    fn get_download(&self, group_name: &str, addon_name: &str, edition: &str) -> Result<Download, Self::Error>;
    fn get_diff(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Option<Diff>, Self::Error>;

    fn get_paths(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Vec<String>, Self::Error>;

    fn get_integrity(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<Vec<IntegrityInfo>, Self::Error>;

    fn has_diff_transition(&self) -> Result<bool, Self::Error>;
    fn run_diff_transition(&self, group_name: &str, addon_name: &str, transition_path: &str, edition: &str) -> Result<(), Self::Error>;

    fn has_diff_post_transition(&self) -> Result<bool, Self::Error>;
    fn run_diff_post_transition(&self, group_name: &str, addon_name: &str, addon_path: &str, edition: &str) -> Result<(), Self::Error>;
}
