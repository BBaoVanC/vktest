use anyhow::Context;
use wayland_client::{protocol::wl_registry::WlRegistry, Connection, Dispatch};

mod render;

struct State;

fn main() -> anyhow::Result<()> {
    let conn = Connection::connect_to_env().context("failed to get wayland connect")?;
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let queue_handle = event_queue.handle();
    let _registry = display.get_registry(&queue_handle, ());

    let vk_ctx = render::vulkan::Context::from_entry(ash::Entry::linked());

    event_queue
        .roundtrip(&mut State)
        .context("error in roundtrip")?;

    Ok(())
}

impl Dispatch<WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: <WlRegistry as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        println!("event: {:?}", event);
    }
}
