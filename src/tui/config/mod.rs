mod default_keymaps;

use crate::{
    prelude::*,
    tui::{actions::Action, config::default_keymaps::default_keymaps},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Deserialize, Serialize, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub project_registries: HashMap<String, String>,
    #[serde(default)]
    pub registry_commands: HashMap<String, RegistryCommands>,
    #[serde(default)]
    pub command_behaviours: CommandBehaviours,
    #[default(default_keymaps())]
    #[serde(default)]
    pub keymaps: Keymaps,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = config_path()?;

        if !config_file.exists() {
            let default_config = Config::create_and_write_default()?;
            return Ok(default_config);
        }

        let config_str =
            fs::read_to_string(config_file).with_context(|| "Failed to read config file")?;

        Ok(serde_json::from_str(&config_str)?)
    }

    pub fn create_backup() -> Result<()> {
        let config_file = config_path()?;
        let mut backup_file = config_file.clone();
        backup_file.set_file_name(format!(
            "config_backup_{}.json",
            Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
        ));
        fs::copy(config_file, backup_file)?;
        Ok(())
    }

    pub fn create_and_write_default() -> Result<Config> {
        let default_config = Config::default();
        let config_file = config_path()?;
        let config_parent = config_file
            .parent()
            .with_context(|| anyhow!("Failed to retrieve config file dirname"))?;

        fs::create_dir_all(config_parent)
            .with_context(|| anyhow!("Failed to create config directory"))?;
        fs::write(config_file, serde_json::to_string_pretty(&default_config)?)
            .with_context(|| anyhow!("Failed to write to config file"))?;

        Ok(default_config)
    }

    pub fn action_keymaps(&self, act: &Action) -> Result<&Vec<KeyMap>> {
        self.keymaps
            .get(act)
            .ok_or(anyhow!("No configured keymaps for {:?}", act))
    }

    pub fn action_triggered(&self, act: &Action, key_event: &KeyEvent) -> bool {
        let Ok(keymaps) = self.action_keymaps(act) else {
            return false;
        };

        for keymap in keymaps {
            if key_event
                .modifiers
                .contains(keymap.modifiers.unwrap_or(KeyModifiers::empty()))
                && key_event.code == keymap.key
            {
                return true;
            }
        }

        false
    }

    pub fn match_actions(&self, key_event: &KeyEvent, actions: &[Action]) -> Action {
        for act in actions {
            if self.action_triggered(act, key_event) {
                return act.clone();
            }
        }
        Action::Noop
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistryCommands {
    pub delete_image: String,
    pub list_images: String,
}

#[derive(Deserialize, Serialize, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct CommandBehaviours {
    #[serde(default)]
    pub push_image: CommandBehaviour,
    #[serde(default)]
    pub delete_image: CommandBehaviour,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum CommandBehaviour {
    Async,
    #[default]
    Interactive,
}

#[derive(Deserialize, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct KeyMap {
    pub key: KeyCode,
    pub modifiers: Option<KeyModifiers>,
}

#[bon]
impl KeyMap {
    #[builder]
    pub fn char(key: char, modifiers: Option<KeyModifiers>) -> Self {
        KeyMap::builder()
            .key(KeyCode::Char(key))
            .maybe_modifiers(modifiers)
            .build()
    }
}

impl From<(Option<KeyModifiers>, KeyCode)> for KeyMap {
    fn from((modifiers, key): (Option<KeyModifiers>, KeyCode)) -> Self {
        Self { modifiers, key }
    }
}

type Keymaps = HashMap<Action, Vec<KeyMap>>;
