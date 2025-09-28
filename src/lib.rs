slint::include_modules!();

mod models;
mod services;
mod viewmodels;

pub fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use i_slint_backend_winit::winit::platform::windows::{
            BackdropType, Color, WindowAttributesExtWindows,
        };
        let mut backend = i_slint_backend_winit::Backend::new()?;
        backend.window_attributes_hook = Some(Box::new(|attrs| {
            attrs
                .with_title_background_color(Some(Color::from_rgb(74, 62, 76)))
                .with_system_backdrop(BackdropType::TransientWindow)
        }));
        slint::platform::set_platform(Box::new(backend))?;
    }
    let token = tokio_util::sync::CancellationToken::new();
    let ui = MainWindow::new()?;
    let join = setup(token.clone(), ui.as_weak())?;

    viewmodels::init()?;

    ui.run()?;
    token.cancel();
    join.join().unwrap();
    Ok(())
}

fn setup(
    token: tokio_util::sync::CancellationToken,
    ui_weak: slint::Weak<MainWindow>,
) -> tokio::io::Result<std::thread::JoinHandle<()>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let rt_handle = rt.handle().clone();
    let join = std::thread::spawn(move || {
        rt.block_on(token.cancelled());
        println!("Tokio Thread closed");
    });
    let spot = rt_handle.block_on(async { services::spotify::SpotifyService::default() });
    services::init(spot, rt_handle, ui_weak);
    Ok(join)
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main().unwrap();
}
