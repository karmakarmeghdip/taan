slint::include_modules!();

mod spotify;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = setup_rt()?;
    let spot = rt.block_on(async { spotify::SpotifyState::default() });
    let ui = MainWindow::new()?;

    ui.run()?;
    Ok(())
}

fn setup_rt() -> tokio::io::Result<tokio::runtime::Handle> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let rt_handle = rt.handle().clone();
    std::thread::spawn(move || {
        rt.block_on(std::future::pending::<()>());
    });
    Ok(rt_handle)
}
