slint::include_modules!();

mod spotify;
mod state;

pub fn main() -> anyhow::Result<()> {
    let rt = setup_rt()?;
    let spot = rt.block_on(async { spotify::SpotifyState::default() });
    let ui = MainWindow::new()?;

    state::setup(ui.clone_strong(), spot.clone(), rt.clone());

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

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    MainWindow::new().unwrap().run().unwrap();
}
