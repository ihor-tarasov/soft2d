#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(u32);

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 0xFF)
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub const fn from_u32(v: u32) -> Self {
        Self(v)
    }

    pub const fn a(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub const BLACK: Self = Self::from_rgb(0x00, 0x00, 0x00);
    pub const WHITE: Self = Self::from_rgb(0xFF, 0xFF, 0xFF);
    pub const RED: Self = Self::from_rgb(0xFF, 0x00, 0x00);
    pub const GREEN: Self = Self::from_rgb(0x00, 0xFF, 0x00);
    pub const BLUE: Self = Self::from_rgb(0x00, 0x00, 0xFF);

    pub const YELLOW: Self = Self::from_rgb(0xFF, 0xFF, 0x00);
    pub const CYAN: Self = Self::from_rgb(0x00, 0xFF, 0xFF);
    pub const MAGENTA: Self = Self::from_rgb(0xFF, 0x00, 0xFF);

    pub const GRAY: Self = Self::from_rgb(0x80, 0x80, 0x80);
    pub const LIGHT_GRAY: Self = Self::from_rgb(0xC0, 0xC0, 0xC0);
    pub const DARK_GRAY: Self = Self::from_rgb(0x40, 0x40, 0x40);

    pub const ORANGE: Self = Self::from_rgb(0xFF, 0xA5, 0x00);
    pub const BROWN: Self = Self::from_rgb(0xA5, 0x2A, 0x2A);
    pub const PINK: Self = Self::from_rgb(0xFF, 0xC0, 0xCB);
    pub const PURPLE: Self = Self::from_rgb(0x80, 0x00, 0x80);
}
