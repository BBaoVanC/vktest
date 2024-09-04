use ash::vk;
use ash::{Entry, Instance};
use std::default::Default;
use snafu::Snafu;
use snafu::prelude::*;
use std::ffi::c_char;

pub struct Context {
    //entry: Entry,
    instance: Instance,
}

#[derive(Debug, Snafu)]
pub enum CreateContextError {
    #[snafu(display("error creating Vulkan instance"))]
    CreateInstanceError { source: vk::Result },
}
impl Context {
    pub fn from_entry(entry: Entry) -> Result<Self, CreateContextError> {
        let app_info = vk::ApplicationInfo {
            // SAFETY: TODO: confirm this is safe
            // https://stackoverflow.com/questions/53611161/how-do-i-expose-a-compile-time-generated-static-c-string-through-ffi
            // OLD: now 1.77 has c string literals -- p_application_name: concat!("vktest", "\0").as_ptr() as *const c_char,
            p_application_name: c"vktest".as_ptr(),
            //application_version: env!("CARGO_PKG_VERSION"),
            application_version: 1, // TODO: get this automatically
            api_version: vk::make_api_version(0, 1, 0, 0), // vulkan 1.0 i think
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&create_info, None).context(CreateInstanceSnafu)? };
        Ok(Self { instance })
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: remember to drop all child objects of instance before drop
        unsafe { };

        unsafe { self.instance.destroy_instance(None) };
    }
}
