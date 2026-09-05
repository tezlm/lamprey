use gpui::colors::Colors;
use gpui::{Global, rgba};

use crate::oklch;
use crate::oklch::Oklch;

#[derive(Clone)]
pub struct Theme {
    fg_100: Oklch,
    fg_200: Oklch,
    fg_300: Oklch,
    fg_400: Oklch,
    fg_500: Oklch,
    fg_600: Oklch,
    bg_100: Oklch,
    bg_150: Oklch,
    bg_200: Oklch,
    bg_250: Oklch,
    bg_300: Oklch,
    bg_400: Oklch,
    sep_100: Oklch,
    sep_300: Oklch,
    sep_400: Oklch,
    sep_800: Oklch,
    link_200: Oklch,
    link_300: Oklch,
    link_400: Oklch,
    link_500: Oklch,
    warn: Oklch,
    error: Oklch,
    status_online: Oklch,
    highlight: Oklch,
    red: Oklch,
    green: Oklch,
    yellow: Oklch,
    blue: Oklch,
    magenta: Oklch,
    cyan: Oklch,
    orange: Oklch,
    teal: Oklch,
}

#[rustfmt::skip]
impl Theme {
    pub fn fg_100(&self) -> Oklch { self.fg_100 }
    pub fn fg_200(&self) -> Oklch { self.fg_200 }
    pub fn fg_300(&self) -> Oklch { self.fg_300 }
    pub fn fg_400(&self) -> Oklch { self.fg_400 }
    pub fn fg_500(&self) -> Oklch { self.fg_500 }
    pub fn fg_600(&self) -> Oklch { self.fg_600 }
    pub fn bg_100(&self) -> Oklch { self.bg_100 }
    pub fn bg_150(&self) -> Oklch { self.bg_150 }
    pub fn bg_200(&self) -> Oklch { self.bg_200 }
    pub fn bg_250(&self) -> Oklch { self.bg_250 }
    pub fn bg_300(&self) -> Oklch { self.bg_300 }
    pub fn bg_400(&self) -> Oklch { self.bg_400 }
    pub fn sep_100(&self) -> Oklch { self.sep_100 }
    pub fn sep_300(&self) -> Oklch { self.sep_300 }
    pub fn sep_400(&self) -> Oklch { self.sep_400 }
    pub fn sep_800(&self) -> Oklch { self.sep_800 }
    pub fn link_200(&self) -> Oklch { self.link_200 }
    pub fn link_300(&self) -> Oklch { self.link_300 }
    pub fn link_400(&self) -> Oklch { self.link_400 }
    pub fn link_500(&self) -> Oklch { self.link_500 }
    pub fn warn(&self) -> Oklch { self.warn }
    pub fn error(&self) -> Oklch { self.error }
    pub fn status_online(&self) -> Oklch { self.status_online }
    pub fn highlight(&self) -> Oklch { self.highlight }
    pub fn red(&self) -> Oklch { self.red }
    pub fn green(&self) -> Oklch { self.green }
    pub fn yellow(&self) -> Oklch { self.yellow }
    pub fn blue(&self) -> Oklch { self.blue }
    pub fn magenta(&self) -> Oklch { self.magenta }
    pub fn cyan(&self) -> Oklch { self.cyan }
    pub fn orange(&self) -> Oklch { self.orange }
    pub fn teal(&self) -> Oklch { self.teal }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            fg_100: oklch!(1.0, 0.0, 0.0),
            fg_200: oklch!(0.949, 0.0, 0.0),
            fg_300: oklch!(0.898, 0.0, 0.0),
            fg_400: oklch!(0.845, 0.0, 0.0),
            fg_500: oklch!(0.738, 0.0, 0.0),
            fg_600: oklch!(0.694, 0.0, 0.0),
            bg_100: oklch!(0.17, 0.008, 229.4),
            bg_150: oklch!(0.1884, 0.009, 229.4),
            bg_200: oklch!(0.223, 0.01, 234.2),
            bg_250: oklch!(0.253, 0.01, 234.2),
            bg_300: oklch!(0.275, 0.011, 225.5),
            bg_400: oklch!(0.34, 0.014, 217.8),
            sep_100: oklch!(0.373, 0.016, 208.8),
            sep_300: oklch!(0.2783, 0.016, 219.8),
            sep_400: oklch!(0.3283, 0.016, 219.8),
            sep_800: oklch!(0.5253, 0.0149, 208.75),
            link_200: oklch!(0.516, 0.15, 299.2),
            link_300: oklch!(0.616, 0.15, 299.2),
            link_400: oklch!(0.72, 0.1169, 299.2),
            link_500: oklch!(0.716, 0.15, 299.2),
            warn: oklch!(0.8594, 0.1563, 89.66),
            error: oklch!(0.602, 0.1976, 20.66),
            status_online: oklch!(0.8042, 0.1367, 154.99),
            highlight: oklch!(0.602, 0.1976, 20.66),
            red: oklch!(0.7403, 0.1759, 13.16),
            green: oklch!(0.8553, 0.1395, 130.14),
            yellow: oklch!(0.8539, 0.1187, 92.43),
            blue: oklch!(0.7929, 0.1636, 255.6),
            magenta: oklch!(0.806, 0.15, 299.2),
            cyan: oklch!(0.8021, 0.1086, 199.72),
            orange: oklch!(0.807, 0.1273, 50.56),
            teal: oklch!(0.8, 0.128, 168.0),
        }
    }
}

impl Global for Theme {}

impl Theme {
    /// get a [gpui::Colors] based on this theme
    pub fn to_gpui_theme(&self) -> Colors {
        Colors {
            text: self.fg_100().to_rgba(),
            selected_text: rgba(0x3FA9C988),
            background: self.bg_100().to_rgba(),
            disabled: self.fg_600().to_rgba(),
            selected: rgba(0x3FA9C9FF),
            border: self.sep_300().to_rgba(),
            separator: self.sep_300().to_rgba(),
            container: self.bg_200().to_rgba(),
        }
    }
}
