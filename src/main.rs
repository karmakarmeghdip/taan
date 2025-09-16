slint::include_modules!();

mod spotify;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let spot = rt.block_on(async { spotify::SpotifyState::default() });
    let rt_handle = rt.handle().clone();
    std::thread::spawn(move || {
        rt.block_on(std::future::pending::<()>());
    });
    let ui = MainWindow::new()?;
    {
        ui.on_clicked(move || {
            let spot = spot.clone();
            println!("Attempting to authenticate...");
            rt_handle.spawn(async move {
                spot.auth().await.unwrap();
                println!("Auth successful");
            });
        });
    }
    ui.run()?;
    Ok(())
}
