use crate::{config::Config, to_cstr};

pub struct Container(pub *mut sys::lxc_container);

impl Container {
    pub unsafe fn new(name: &'static str, config: Config) -> Self {
        let inner = unsafe { sys::lxc_container_new(to_cstr!(name), config.to_cstr()) };
        Self(inner)
    }
    pub fn as_ptr(&self) -> *const sys::lxc_container {
        self.0 as *const _
    }
    pub fn as_mut_ptr(&self) -> *mut sys::lxc_container {
        self.0
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe { sys::lxc_container_put(self.0) };
    }
}