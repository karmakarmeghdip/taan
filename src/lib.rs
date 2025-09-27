slint::include_modules!();

mod models;
mod services;
mod viewmodels;

pub static RT: std::sync::OnceLock<tokio::runtime::Handle> = std::sync::OnceLock::new();
pub static UI: std::sync::OnceLock<slint::Weak<MainWindow>> = std::sync::OnceLock::new();

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
    let join = setup_rt(token.clone())?;

    let ui = MainWindow::new()?;

    let ui_weak = ui.as_weak();
    UI.set(ui_weak).unwrap_or_else(|_| {
        eprintln!("Failed to set UI weak reference");
    });

    viewmodels::window_vm::register_handlers()?;
    viewmodels::authentication_vm::init();
    viewmodels::authentication_vm::register_handlers()?;
    viewmodels::player_vm::register_handlers()?;

    ui.run()?;
    token.cancel();
    join.join().unwrap();
    Ok(())
}

fn setup_rt(
    token: tokio_util::sync::CancellationToken,
) -> tokio::io::Result<std::thread::JoinHandle<()>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let rt_handle = rt.handle().clone();
    let join = std::thread::spawn(move || {
        rt.block_on(token.cancelled());
        println!("Tokio Thread closed");
    });
    rt_handle.block_on(async {
        services::spotify::SpotifyService::default().register();
    });
    RT.set(rt_handle).unwrap();
    Ok(join)
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main().unwrap();
}
