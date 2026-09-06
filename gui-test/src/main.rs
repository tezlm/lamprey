use std::sync::Arc;

use gpui::{colors::GlobalColors, *};
use gui_test::theme::Theme;

#[derive(Default)]
struct HelloWorld;

actions!(global, [Quit]);

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme: &Theme = cx.global();

        let room_header = div()
            .h_12()
            .p_2()
            .items_center()
            .bg(theme.bg_150())
            .border_b_1()
            .border_color(theme.sep_300())
            .child("room name");
        let nav_room = div().bg(theme.bg_100()).w(px(64.));
        let nav_channel = div().bg(theme.bg_150()).w(px(256.)).child(room_header);
        let tray = div()
            .bg(theme.bg_100())
            .border_t_1()
            .border_color(theme.sep_300())
            .p_2()
            .child("user tray");
        let nav_tray = div()
            .flex()
            .flex_col()
            .w(px(320.))
            .border_r_1()
            .border_color(theme.sep_300())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .child(nav_room)
                    .child(nav_channel),
            )
            .child(tray);

        let header = div()
            .h_12()
            .p_2()
            .items_center()
            .bg(theme.bg_150())
            .border_b_1()
            .border_color(theme.sep_300())
            .child("header");

        let main = div()
            .bg(theme.bg_200())
            .flex_1()
            .child("hello, world!")
            .on_mouse_down(MouseButton::Left, |e, _window, _cx| {
                println!("mouse down {e:?}");
            })
            .on_key_down(|e, _window, _cx| {
                println!("key down {e:?}");
            })
            .tab_group();

        let sidebar = div()
            .w(px(198.))
            .bg(theme.bg_150())
            .border_l_1()
            .border_color(theme.sep_300());

        let root = div()
            .size_full()
            .text_color(theme.fg_100())
            .flex()
            .flex_col()
            .child(
                div().flex().flex_row().flex_1().child(nav_tray).child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .child(header)
                        .child(div().flex().flex_row().flex_1().child(main).child(sidebar)),
                ),
            );

        root
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

        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|_| HelloWorld::default())
        })
        .unwrap();
    });
}
