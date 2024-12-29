#![allow(unused)]

use std::path::PathBuf;

use dirs::data_local_dir;
use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Icon, Window, WindowBuilder};
use wry::{WebContext, WebView, WebViewBuilder};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "icons/"]
struct Asset;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    event_loop.set_device_event_filter(tao::event_loop::DeviceEventFilter::Never);

    let icon = load_icon();

    let window = WindowBuilder::new()
        .with_title("DI.FM")
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "linux"))]
    let _webview = {
        let builder = WebViewBuilder::new().with_url("https://di.fm");
        builder.build(&window)?
    };

    //Linux specific build
    #[cfg(target_os = "linux")]
    let _webview = {
        let mut data_dir = None::<PathBuf>;
        if let Some(mut local_dir) = data_local_dir() {
            local_dir.push("difm-player");
            data_dir = Some(local_dir);
        }
        let mut context = WebContext::new(data_dir);
        let builder = WebViewBuilder::with_web_context(&mut context).with_url("https://di.fm");
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let asset = Asset::get("icon.png").unwrap();
        let raw = asset.data.as_ref();
        let image = image::load_from_memory(raw)
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
