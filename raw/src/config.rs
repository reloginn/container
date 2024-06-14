use std::path::Path;

use crate::to_cstr;

pub struct Config(pub(crate) &'static Path);

impl Default for Config {
    fn default() -> Self {
        Self::new(std::path::Path::new("/etc/lxc/default.conf"))
    }
}

impl Config {
    pub fn new(path: &'static Path) -> Self {
        if path.exists() {
            Self(path)
        } else {
            panic!("The config does not exist, please try using [`Config::default`]")
        }
    }
    pub(crate) fn to_cstr(&self) -> *const i8 {
        let s = self.0.to_str().unwrap();
        to_cstr!(s)
    }
}
