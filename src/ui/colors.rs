use floem::peniko::Color;

#[derive(Copy, Clone)]
pub struct Accents {
    pub color1: Color,
    pub color2: Color,
    pub color3: Color,
    pub color4: Color,
    pub color5: Color,
    pub color6: Color,
    pub color7: Color,
    pub color8: Color,
    pub color9: Color,
    pub color10: Color,
    pub color11: Color,
    pub danger: Color,
    pub warning: Color,
}

impl Default for Accents {
    fn default() -> Self {
        Accents {
            color1: Color::parse("#75BEFF").unwrap(),
            color2: Color::parse("#40A6FF").unwrap(),
            color3: Color::parse("#3399CC").unwrap(),
            color4: Color::parse("#3794FF").unwrap(),
            color5: Color::parse("#0097FB").unwrap(),
            color6: Color::parse("#007ACC").unwrap(),
            color7: Color::parse("#0E639C").unwrap(),
            color8: Color::parse("#264F78").unwrap(),
            color9: Color::parse("#094771").unwrap(),
            color10: Color::parse("#062F4A").unwrap(),
            color11: Color::parse("#001F33").unwrap(),
            danger: Color::parse("#DC3545").unwrap(),
            warning: Color::parse("#FFC107").unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Base {
    pub base1: Color,
    pub base2: Color,
    pub base3: Color,
    pub base4: Color,
    pub base5: Color,
    pub base6: Color,
    pub base7: Color,
    pub base8: Color,
    pub base9: Color,
    pub base10: Color,
    pub base11: Color,
    pub base12: Color,
    pub base13: Color,
    pub base14: Color,
    pub base15: Color,
    pub base16: Color,
    pub base17: Color,
    pub base18: Color,
    pub base19: Color,
    pub base20: Color,
    pub base21: Color,
}

impl Default for Base {
    fn default() -> Self {
        Base {
            base1: Color::parse("#FFFFFF").unwrap(),
            base2: Color::parse("#F0F0F0").unwrap(),
            base3: Color::parse("#E7E7E7").unwrap(),
            base4: Color::parse("#E5E5E5").unwrap(),
            base5: Color::parse("#D4D4D4").unwrap(),
            base6: Color::parse("#CCCCCC").unwrap(),
            base7: Color::parse("#C6C6C6").unwrap(),
            base8: Color::parse("#BBBBBB").unwrap(),
            base9: Color::parse("#A0A0A0").unwrap(),
            base10: Color::parse("#808080").unwrap(),
            base11: Color::parse("#7F7F7F").unwrap(),
            base12: Color::parse("#606060").unwrap(),
            base13: Color::parse("#454545").unwrap(),
            base14: Color::parse("#3C3C3C").unwrap(),
            base15: Color::parse("#3A3D41").unwrap(),
            base16: Color::parse("#333333").unwrap(),
            base17: Color::parse("#303031").unwrap(),
            base18: Color::parse("#292929").unwrap(),
            base19: Color::parse("#252526").unwrap(),
            base20: Color::parse("#1E1E1E").unwrap(),
            base21: Color::parse("#000000").unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ColorStyles {
    pub accent: Accents,
    pub base: Base,
}

impl Default for ColorStyles {
    fn default() -> Self {
        ColorStyles {
            accent: Accents::default(),
            base: Base::default(),
        }
    }
}
