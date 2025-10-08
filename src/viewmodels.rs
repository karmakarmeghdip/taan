pub mod authentication_vm;
pub mod player_vm;
pub mod utils;
pub mod window_vm;

pub fn init() -> anyhow::Result<()> {
    window_vm::register_handlers()?;
    authentication_vm::init();
    authentication_vm::register_handlers()?;
    player_vm::register_handlers()?;
    utils::register_handlers()?;
    Ok(())
}
