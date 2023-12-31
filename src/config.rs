use rdev::Key;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub channel: u8,
    pub keymap: Vec<KeyMap>,
}

#[derive(Deserialize, Debug)]
pub struct KeyMap {
    pub key: Key,
    pub note: u8,
}

impl Config {
    pub fn get_key(&self, key: Key) -> Option<u8> {
        self.keymap.iter().find(|x| x.key == key).map(|x| x.note)
    }
}
