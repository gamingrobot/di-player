//#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute

use rust_embed::Embed;

use muda::{
    Menu, MenuEvent, MenuItem, Submenu,
};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Icon, WindowBuilder},
    dpi::{LogicalSize},
};
use wry::WebViewBuilder;

#[derive(Embed)]
#[folder = "icons/"]
struct Asset;

enum UserEvent {
    MenuEvent(muda::MenuEvent),
}

fn main() -> wry::Result<()> {
    let mut event_loop_builder = EventLoopBuilder::<UserEvent>::with_user_event();

    let menu_bar = Menu::new();

    // setup accelerator handler on Windows
    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::EventLoopBuilderExtWindows;
        let menu_bar = menu_bar.clone();
        event_loop_builder.with_msg_hook(move |msg| {
            use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
            unsafe {
                let msg = msg as *const MSG;
                let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel() as _, msg);
                translated == 1
            }
        });
    }

    let event_loop = event_loop_builder.build();

    // set a menu event handler that wakes up the event loop
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let icon = load_icon("icon.png");

    let window = WindowBuilder::new()
        .with_title("DI Player")
        .with_window_icon(Some(icon))
        .with_inner_size(LogicalSize::new(1024.0, 1024.0))
        .build(&event_loop)
        .unwrap();

    let station_m = Submenu::new("&Station", true);

    menu_bar.append_items(&[&station_m]).unwrap();

    let station_difm = MenuItem::new("DI.FM", true, None);
    let station_radiotunes = MenuItem::new("RadioTunes", true, None);
    let station_zen = MenuItem::new("Zen Radio", true, None);
    let station_rockradio = MenuItem::new("Rock Radio", true, None);
    let station_classical = MenuItem::new("Classical Radio", true, None);
    let station_jazz = MenuItem::new("Jazz Radio", true, None);

    station_m
        .append_items(&[
            &station_difm,
            &station_radiotunes,
            &station_zen,
            &station_rockradio,
            &station_classical,
            &station_jazz,
        ])
        .unwrap();

    #[cfg(target_os = "windows")]
    unsafe {
        use tao::platform::windows::WindowExtWindows;
        menu_bar.init_for_hwnd(window.hwnd() as _).unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        use tao::platform::unix::WindowExtUnix;
        menu_bar.init_for_gtk_window(window.gtk_window(), window.default_vbox()).unwrap();
    }

    #[cfg(not(target_os = "linux"))]
    let webview = {
        let builder = WebViewBuilder::new().with_url("https://www.di.fm");
        builder.build(&window)?
    };

    //Linux specific build
    #[cfg(target_os = "linux")]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::{WebContext, WebViewBuilderExtUnix};
        use std::path::PathBuf;
        use dirs::data_local_dir;
        let mut data_dir = None::<PathBuf>;
        if let Some(mut local_dir) = data_local_dir() {
            local_dir.push("di-player");
            data_dir = Some(local_dir);
        }
        let mut context = WebContext::new(data_dir);
        let builder = WebViewBuilder::with_web_context(&mut context).with_url("https://www.di.fm");
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == station_difm.id() {
                    let _ = webview.load_url("https://www.di.fm");
                } else if event.id == station_radiotunes.id() {
                    let _ = webview.load_url("https://www.radiotunes.com");
                } else if event.id == station_zen.id() {
                    let _ = webview.load_url("https://www.zenradio.com");
                } else if event.id == station_rockradio.id() {
                    let _ = webview.load_url("https://www.rockradio.com");
                } else if event.id == station_classical.id() {
                    let _ = webview.load_url("https://www.classicalradio.com");
                } else if event.id == station_jazz.id() {
                    let _ = webview.load_url("https://www.jazzradio.com");
                }
            }
            _ => {}
        }
    })
}

fn load_icon(asset_name: &str) -> Icon {
    let (icon_rgba, icon_width, icon_height) = load_icon_raw(asset_name);
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_icon_raw(asset_name: &str) -> (Vec<u8>, u32, u32) {
        let asset = Asset::get(asset_name).unwrap();
        let raw = asset.data.as_ref();
        let image = image::load_from_memory(raw)
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
}

