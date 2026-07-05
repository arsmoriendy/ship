mod default_keymaps;

use crate::{
    prelude::*,
    tui::{actions::Action, config::default_keymaps::DEFAULT_KEYMAP_STR},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_registries: HashMap<String, String>,
    pub registry_commands: HashMap<String, RegistryCommands>,
    pub command_behaviours: CommandBehaviours,
    pub keymaps: Keymaps,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = config_path()?;

        if !config_file.exists() {
            let config_parent = config_file
                .parent()
                .with_context(|| anyhow!("failed to retrieve config file dirname"))?;
            fs::create_dir_all(config_parent)?;
            let config = Config {
                keymaps: Self::default_keymaps()?,
                ..Default::default()
            };
            fs::write(config_file, serde_json::to_string_pretty(&config)?)?;
            return Ok(config);
        }

        let config_str =
            fs::read_to_string(config_file).with_context(|| "failed to read config file")?;
        serde_json::from_str(&config_str).with_context(|| "failed to parse config file")
    }

    pub fn default_keymaps() -> Result<Keymaps> {
        Ok(serde_json::from_str(DEFAULT_KEYMAP_STR)?)
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
    pub list_digests: String,
}

#[derive(Deserialize, Serialize, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct CommandBehaviours {
    #[default(CommandBehaviour::Interactive)]
    pub push_image: CommandBehaviour,
    pub delete_image: CommandBehaviour,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum CommandBehaviour {
    #[default]
    Async,
    Interactive,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyMap {
    pub key: KeyCode,
    pub modifiers: Option<KeyModifiers>,
}

impl From<(Option<KeyModifiers>, KeyCode)> for KeyMap {
    fn from((modifiers, key): (Option<KeyModifiers>, KeyCode)) -> Self {
        Self { modifiers, key }
    }
}

type Keymaps = HashMap<Action, Vec<KeyMap>>;
