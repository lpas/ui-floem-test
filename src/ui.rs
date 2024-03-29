pub mod colors;
// mod texts;

use std::fmt::Display;

use self::colors::ColorStyles;

use floem::peniko::Color;
use floem::reactive::create_rw_signal;
use floem::style::{CursorStyle, Style};
use floem::views::{self, container, svg};
use floem::widgets::dropdown::dropdown;
use floem::widgets::{
    dropdown, ButtonClass, ListClass, ListItemClass, PlaceholderTextClass, TextInputClass,
};

use floem::{
    style_class,
    view::View,
    views::{label, stack, Decorators},
};
use strum::IntoEnumIterator;

style_class!(pub ButtonSecondary);
style_class!(pub ButtonSmall);

pub struct UI {
    pub color_styles: ColorStyles,
    pub style: Style,
}

// todo specify inter font :: missing support at https://github.com/lapce/floem/issues/76
impl UI {
    fn style(color_styles: ColorStyles) -> Style {
        let button = Style::new()
            .background(color_styles.accent.color7)
            .color(color_styles.base.base1)
            // .border(0)
            .border_radius(1.0)
            .padding_horiz(12.0)
            .padding_vert(6.0)
            .hover(|s| s.background(color_styles.accent.color6))
            .disabled(|s| {
                s.background(color_styles.accent.color7.with_alpha_factor(0.4))
                    .color(color_styles.base.base1.with_alpha_factor(0.4))
            })
            .focus_visible(|s| {
                s.outline(1.0).outline_color(color_styles.accent.color6) // todo this is not quite right we need a 1 px gap between button and outline
            })
            .height(30)
            .font_size(13.0); // todo b1

        let button_secondary = Style::new()
            .background(color_styles.base.base15)
            .color(color_styles.base.base1)
            // .border(0)
            .border_radius(1.0)
            .padding_horiz(12.0)
            .padding_vert(6.0)
            .hover(|s| s.background(color_styles.base.base13))
            .disabled(|s| {
                s.background(color_styles.base.base15.with_alpha_factor(0.4))
                    .color(color_styles.base.base1.with_alpha_factor(0.4))
            })
            .focus_visible(|s| {
                s.outline(1.0).outline_color(color_styles.accent.color6) // todo this is not quite right we need a 1 px gap between button and outline
            })
            .height(30)
            .font_size(13.0); // todo b1

        let button_small = Style::new()
            .background(color_styles.base.base20)
            .color(color_styles.base.base6)
            .border(1)
            .border_color(color_styles.base.base15)
            .border_radius(1.0)
            .padding_horiz(12.0)
            .padding_vert(4.5)
            .hover(|s| s.background(color_styles.base.base17))
            .disabled(|s| {
                s.background(color_styles.base.base20.with_alpha_factor(0.4))
                    .color(color_styles.base.base6.with_alpha_factor(0.4))
            })
            .focus_visible(|s| s.border_color(color_styles.accent.color6))
            .height(24)
            .font_size(12.0); // todo doesn't looks quite right // todo b2

        let input_style = Style::new()
            .background(color_styles.base.base16)
            .border_radius(1)
            .border(1)
            .border_color(color_styles.base.base13)
            .color(color_styles.base.base3)
            .focus(|s| s.border_color(color_styles.accent.color6))
            // todo text selection color
            .cursor(CursorStyle::Text)
            .padding_vert(2.0) // todo should be 4 something is off
            .padding_horiz(8)
            .height(24)
            .font_size(13.0) // todo b1
            .cursor_color(color_styles.accent.color9.with_alpha_factor(0.4)); // todo define this in figma // todo this seems kinda buggy should be painted behind te
                                                                              // todo define disabled style in figma

        let dropdown_label = Style::new()
            .background(color_styles.base.base16)
            .border_radius(1)
            .border(1)
            .border_color(color_styles.base.base13)
            .color(color_styles.base.base3)
            .focus(|s| s.border_color(color_styles.accent.color6))
            // todo text selection color
            .padding_vert(4.0)
            .padding_horiz(8)
            .height(24)
            .font_size(13.0) // todo b1
            .cursor_color(color_styles.accent.color9.with_alpha_factor(0.4)) // todo define this in figma // todo this seems kinda buggy should be painted behind te
            .width_full();
        // todo define disabled style in figma

        Style::new()
            .background(color_styles.base.base20)
            // .transition(Background, Transition::linear(0.5))
            .class(ButtonClass, move |_| button)
            .class(ButtonSecondary, move |_| button_secondary)
            .class(ButtonSmall, move |_| button_small)
            .class(TextInputClass, move |_| input_style)
            .class(dropdown::DropDownClass, |_| {
                // todo we need an open color blue border
                dropdown_label.class(ListClass, |s| {
                    // todo show active item
                    s.width_full()
                        .background(color_styles.base.base20)
                        .border_radius(1) // todo we only want radius on the bottom & the input should not have the bottom radius
                        .border(1)
                        .border_color(color_styles.accent.color6)
                        .items_center()
                        .class(ListItemClass, |s| {
                            s.padding_vert(1.5)
                                .padding_horiz(8)
                                .font_size(11.0) // todo b3
                                .color(color_styles.base.base6)
                                .hover(|s| s.background(color_styles.base.base16))
                                .selected(|s| {
                                    s.background(color_styles.accent.color7)
                                        .color(color_styles.base.base1)
                                }) // todo find out how to adjust this that it works!
                        })
                })
            })
            .class(PlaceholderTextClass, |s| s.color(color_styles.base.base9))
            .color(color_styles.base.base1)
            .font_size(13.0)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for UI {
    fn default() -> Self {
        let color_styles = ColorStyles::default();
        let style = Self::style(color_styles);
        Self {
            color_styles,
            style,
        }
    }
    // pub const TEXT_STYLES: TextStyles = TextStyles::DEFAULT;
}

pub const CHEVRON_DOWN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 185.344 185.344">
  <path fill="#010002" d="M92.672 144.373a10.707 10.707 0 0 1-7.593-3.138L3.145 59.301c-4.194-4.199-4.194-10.992 0-15.18a10.72 10.72 0 0 1 15.18 0l74.347 74.341 74.347-74.341a10.72 10.72 0 0 1 15.18 0c4.194 4.194 4.194 10.981 0 15.18l-81.939 81.934a10.694 10.694 0 0 1-7.588 3.138z"/>
</svg>"##;

pub(crate) fn dropdown_widget<
    T: 'static
        + std::default::Default
        + strum::IntoEnumIterator
        + std::fmt::Display
        + std::clone::Clone
        + std::marker::Copy,
>() -> floem::widgets::dropdown::DropDown<T>
where
    <T as IntoEnumIterator>::Iterator: Clone,
{
    let show_dropdown = create_rw_signal(false);
    let main_drop_view = move |item| {
        stack((
            label(move || item),
            svg(|| String::from(CHEVRON_DOWN)).style(|s| s.size(12, 12).color(Color::WHITE)), // todo make color configurable
        ))
        .style(|s| s.items_center().justify_between().size_full())
        .any()
    };
    // todo would need a show_dropdown style - blue border
    let dropdown_widget = (move || {
        dropdown(
            move || T::default(),
            main_drop_view,
            T::iter(),
            |item| label(move || item).any(),
        )
        .show_list(move || show_dropdown.get())
        .on_accept(move |_val| show_dropdown.set(false))
    })();
    dropdown_widget
}

pub fn button_secondary<S: Display + 'static>(label: impl Fn() -> S + 'static) -> impl View {
    container(views::label(label))
        .keyboard_navigatable()
        .class(ButtonSecondary)
}

const PLUS: &str = r##"
<svg width="9" height="8" viewBox="0 0 9 8" fill="none" xmlns="http://www.w3.org/2000/svg">
    <line x1="4.68628" x2="4.68628" y2="8" stroke="#CCCCCC"/>
    <line x1="0.686279" y1="4" x2="8.68628" y2="4" stroke="#CCCCCC"/>
</svg>
"##;

pub fn button_small<S: Display + 'static>(label: impl Fn() -> S + 'static) -> impl View {
    stack((
        svg(|| String::from(PLUS)).style(|s| s.size(9, 8).color(Color::WHITE)),
        views::label(label),
    ))
    .style(|s| s.items_center().justify_center().gap(8, 0))
    .keyboard_navigatable()
    .class(ButtonSmall)
}
