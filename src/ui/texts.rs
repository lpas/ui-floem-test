
pub struct Label {
    pub l1: f64,
}

impl Label {
    pub const DEFAULT: Self = Self { l1: 14. };
}

pub struct Body {
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
}

impl Body {
    pub const DEFAULT: Self = Self {
        b1: 13.,
        b2: 12.,
        b3: 11.,
    };
}

pub struct Heading {
    pub h1: f64,
    pub h3: f64,
}

impl Heading {
    pub const DEFAULT: Self = Self {
        h1: 26.,
        h3: 12.,
    };
}

pub struct TextStyles {
    pub label: Label,
    pub body: Body,
    pub heading: Heading,
}

impl TextStyles {
    pub const DEFAULT: Self = Self {
        label: Label::DEFAULT,
        body: Body::DEFAULT,
        heading: Heading::DEFAULT,
    };
}
