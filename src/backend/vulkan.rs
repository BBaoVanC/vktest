use ash::Instance;

struct VkContext {
    vk_instance: Instance,
}
impl Drop for VkContext {
    fn drop(&mut self) {
        // SAFETY: drop all child objects of instance before drop
        unsafe { self.vk_instance.destroy_instance(None) };
    }
}
