use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::pack::config::server_info::ServerInfo;

impl FromCliInput for ServerInfo {
    fn from_cli_input() -> anyhow::Result<Self> {
        let server_info = ServerInfo {
            name: dialoguer::Input::new()
                .with_prompt("Server Name")
                .interact_text()?,
            address: dialoguer::Input::new()
                .with_prompt("Server Address (e.g., example.com:443)")
                .interact_text()?,
            port: dialoguer::Input::new()
                .with_prompt("Server Port")
                .default(2302)
                .interact_text()?,
            password: dialoguer::Input::new()
                .with_prompt("Server Password")
                .allow_empty(true)
                .interact_text()?,
            uses_battle_eye: dialoguer::Confirm::new()
                .with_prompt("Does the server use BattleEye?")
                .default(true)
                .interact()?,
        };

        Ok(server_info)
    }
}
