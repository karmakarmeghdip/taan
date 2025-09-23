#[cfg(target_os = "windows")]
use i_slint_backend_winit::winit::platform::windows::BackdropType::TransientWindow;
#[cfg(target_os = "windows")]
use i_slint_backend_winit::winit::platform::windows::{Color, WindowAttributesExtWindows};

slint::include_modules!();

mod spotify;
mod state;

pub fn main() -> anyhow::Result<()> {
    let mut backend = i_slint_backend_winit::Backend::new()?;
    #[cfg(target_os = "windows")]
    {
        backend.window_attributes_hook = Some(Box::new(|attrs| {
            attrs
                .with_title_background_color(Some(Color::from_rgb(74, 62, 76)))
                .with_system_backdrop(TransientWindow)
        }));
    }
    slint::platform::set_platform(Box::new(backend))?;
    let token = tokio_util::sync::CancellationToken::new();
    let (rt, join) = setup_rt(token.clone())?;
    let spot = rt.block_on(async { spotify::SpotifyState::default() });
    let ui = MainWindow::new()?;

    state::setup(ui.clone_strong(), spot.clone(), rt.clone());

    ui.run()?;
    token.cancel();
    join.join().unwrap();
    Ok(())
}

fn setup_rt(
    token: tokio_util::sync::CancellationToken,
) -> tokio::io::Result<(tokio::runtime::Handle, std::thread::JoinHandle<()>)> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let rt_handle = rt.handle().clone();
    let join = std::thread::spawn(move || {
        rt.block_on(token.cancelled());
        println!("Tokio Thread closed");
    });
    Ok((rt_handle, join))
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main().unwrap();
}
