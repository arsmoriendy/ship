use crate::tui::config::Keymaps;

macro_rules! c_code {
    ($char:expr) => {{
        use crate::tui::config::KeyMap;
        KeyMap::char().key($char).call()
    }};
    ($char:expr, $mod:expr) => {{
        use crate::tui::config::KeyMap;
        KeyMap::char().key($char).modifiers($mod).call()
    }};
}

macro_rules! k_code {
    ($code:expr) => {{
        use crate::tui::config::KeyMap;
        KeyMap::builder().key($code).build()
    }};
    ($char:expr, $mod:expr) => {{
        use crate::tui::config::KeyMap;
        KeyMap::builder().key($code).modifiers($mod).build()
    }};
}

macro_rules! keymaps {
    ($($act:expr => {$($map:expr),+$(,)*})+) => {{
        use crate::tui::config::Keymaps;
        let mut keymaps = Keymaps::new();
        $(keymaps.insert($act, vec![$($map),+]);)+
        keymaps
    }};
}

pub fn default_keymaps() -> Keymaps {
    use crate::tui::config::Action::*;
    use crossterm::event::{KeyCode::*, KeyModifiers as M};

    let keymaps = keymaps!(
        SelectUp => {
            c_code!('k'),
            k_code!(Up),
        }
        SelectDown => {
            c_code!('j'),
            k_code!(Down),
        }
        FocusImages => {
            c_code!('J'),
        }
        FocusProjects => {
            c_code!('K'),
        }
        PushImage => {
            c_code!('P'),
        }
        PullImage => {
            c_code!('p'),
        }
        DeleteRemoteImage => {
            c_code!('D'),
        }
        FetchImages => {
            c_code!('f'),
        }
        PruneRemoteImages => {
            c_code!('P'),
        }
        Quit => {
            c_code!('c', M::CONTROL),
            c_code!('q'),
        }
        ClosePopup => {
            c_code!('c', M::CONTROL),
            c_code!('q'),
        }
    );

    keymaps
}
