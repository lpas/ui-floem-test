mod ui;

use floem::cosmic_text::Weight;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::kurbo::Size;
use floem::peniko::Color;
use floem::reactive::create_rw_signal;
use floem::style::AlignItems;
use floem::views::{container, svg, v_stack, Container};
use floem::widgets::{button, text_input};
use floem::window::WindowConfig;
use floem::Application;

use floem::{
    view::View,
    views::{h_stack, text, Decorators},
};
use ui::colors::ColorStyles;
use ui::{dropdown_widget, UI};

use crate::ui::{button_secondary, button_small};

// const SIDEBAR_WIDTH: f64 = 100.0;

#[derive(strum::EnumIter, Debug, PartialEq, Clone, Copy)]
enum DatabaseConnectionTypes {
    Postgres,
    Mysql,
}

impl Default for DatabaseConnectionTypes {
    fn default() -> Self {
        Self::Postgres
    }
}

impl std::fmt::Display for DatabaseConnectionTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

fn app_view(color_styles: ColorStyles) -> impl View {
    let name = create_rw_signal("My Acme Database".to_string());
    let host = create_rw_signal("localhost".to_string());
    let port = create_rw_signal("5432".to_string()); // todo would need to check to only allow numbers
    let user = create_rw_signal("root".to_string());
    let password = create_rw_signal("root".to_string());
    let default_database = create_rw_signal("acme".to_string());

    let search = create_rw_signal("".to_string());

    // let sidebar_width = create_rw_signal(SIDEBAR_WIDTH);
    // Create a vertical layout

    fn colored_icon(color: String) -> Container {
        // todo this is super hacky
        container(text("").style(move |s| {
            s.width(16)
                .font_size(1.0)
                .height(16)
                .background(Color::parse(&color).unwrap())
        }))
    }

    let color5 = Color::parse("#0097FB").unwrap();
    let color9 = Color::parse("#094771").unwrap();
    // todo text inputs seem to be limited in painted text length... ?
    let left = container(
        v_stack((
            v_stack((
                text("CONNECTIONS"),
                text_input(search).placeholder("Search…"),
            ))
            .style(|s| s.gap(0, 12).padding_horiz(8).padding_top(8)),
            v_stack((
                // todo group & hover for tree type here
                h_stack((colored_icon("#1B9E77".to_string()), text("My first DB")))
                    .style(|s| s.padding_horiz(8).padding_vert(4).gap(8, 0)),
                h_stack((
                    colored_icon("#E7298A".to_string()),
                    text("My Acme Database"),
                ))
                .style(move |s| {
                    s.padding_horiz(8)
                        .padding_vert(4)
                        .gap(8, 0)
                        .background(color9)
                        .border(1)
                        .border_color(color5)
                }),
                h_stack((colored_icon("#7570B3".to_string()), text("Some Random One")))
                    .style(|s| s.padding_horiz(8).padding_vert(4).gap(8, 0)),
                h_stack((
                    svg(|| String::from(ui::CHEVRON_DOWN))
                        .style(|s| s.size(12, 12).margin_horiz(2).color(Color::WHITE)),
                    text("Group"),
                ))
                .style(|s| s.padding_horiz(8).padding_vert(4).gap(8, 0)),
                h_stack((colored_icon("#7570B3".to_string()), text("Some Random One"))).style(
                    |s| {
                        s.padding_horiz(8)
                            .padding_left(32)
                            .padding_vert(4)
                            .gap(8, 0)
                    },
                ),
            ))
            .style(|s| s.size_full()),
            h_stack((
                button_small(|| "New Connection").style(|s| s.width_full()),
                button_small(|| "Group"),
            ))
            .style(|s| s.gap(8, 0).padding(8)), // todo mini buttons + icons
        ))
        .style(|s| s.size_full().gap(0, 8)),
    )
    .style(|s| s.min_width(300));
    let right = container(
        v_stack((
            text("Connection").style(|s| s.font_size(26.0).font_weight(Weight::SEMIBOLD)),
            h_stack((
                v_stack((text("Name"), text_input(name))).style(|s| s.width_full().gap(0, 4)),
                container(text("").style(|s| {
                    s.width(48)
                        .height(24)
                        .background(Color::parse("#E7298A").unwrap())
                })),
            ))
            .style(|s| s.gap(8, 0).align_items(AlignItems::End)),
            v_stack((
                text("Connection Type"),
                dropdown_widget::<DatabaseConnectionTypes>(),
            ))
            .style(|s| s.width_full().gap(0, 4)),
            h_stack((
                v_stack((text("Host"), text_input(host))).style(|s| s.width_full().gap(0, 4)),
                v_stack((text("Port"), text_input(port))).style(|s| s.width(80).gap(0, 4)),
            ))
            .style(|s| s.gap(8, 0)),
            h_stack((
                v_stack((text("User"), text_input(user))).style(|s| s.width_full().gap(0, 4)),
                v_stack((text("Password"), text_input(password))) // todo should not be input but password field
                    .style(|s| s.width_full().gap(0, 4)),
            ))
            .style(|s| s.gap(8, 0)),
            v_stack((text("Default Database"), text_input(default_database)))
                .style(|s| s.width_full().gap(0, 4)),
            h_stack((
                button_secondary(|| "Save"),
                button_secondary(|| "Test"),
                button(|| "Save & Connect"),
            ))
            .style(|s| s.gap(8, 0).justify_end()),
        ))
        .style(|s| s.width(560).padding_horiz(8).padding_vert(24).gap(0, 16)),
    )
    .style(move |s| {
        s.width_full()
            .background(color_styles.base.base19)
            .justify_center()
            .border_left(1)
            .border_color(color_styles.base.base15)
            .padding(8)
    });

    let content = h_stack((left, right)).style(|s| s.size_full());

    let view = container(content);

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn main() {
    let ui = UI::new();

    Application::new()
        .window(
            move |_| {
                app_view(ui.color_styles)
                    .style(move |_| ui.style.clone().width_full().height_full())
            },
            Some(
                WindowConfig::default()
                    .size(Size::new(1280.0, 832.0))
                    .title("floem ui test")
                    .position((3200.0, 300.0).into())
                    .apply_default_theme(false),
            ),
        )
        .run();
}
