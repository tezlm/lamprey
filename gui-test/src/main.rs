use std::sync::Arc;

use gpui::{colors::GlobalColors, *};
use gui_test::{oklch, theme::Theme};

#[derive(Default)]
struct HelloWorld;

actions!(global, [Quit]);

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme: &Theme = cx.global();

        div()
            .flex_col()
            .bg(theme.green())
            .child("hello, world!")
            .on_mouse_down(MouseButton::Left, |e, _window, _cx| {
                println!("mouse down {e:?}");
            })
            .on_key_down(|e, _window, _cx| {
                println!("key down {e:?}");
            })
    }
}

fn main() {
    Application::new().run(|cx| {
        // cx.register_url_scheme("lamprey").detach_and_log_err(cx);
        cx.bind_keys([KeyBinding::new("q", Quit, None)]);
        cx.on_action(|_: &Quit, cx| {
            println!("quit");
            cx.shutdown();
        });

        let theme = Theme::default();
        cx.set_global(GlobalColors(Arc::new(theme.to_gpui_theme())));
        cx.set_global(theme);

        // cx.set_menus(vec![
        //     Menu { name: "asdf".into(), items: vec![MenuItem::Action { name: (), action: (), os_action: () }] }
        // ]);

        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_| HelloWorld::default())
        })
        .unwrap();
    });
}
