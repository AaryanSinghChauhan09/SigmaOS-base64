//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    NoWrap,
    Wrap,
}

#[derive(Debug, Clone)]
pub struct Base64Options {
    pub wrap_mode: WrapMode,
    pub decode: bool,
    pub ignore_garbage: bool,
}

impl Default for Base64Options {
    fn default() -> Self {
        Self {
            wrap_mode: WrapMode::NoWrap,
            decode: false,
            ignore_garbage: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_options() {
        let o = Base64Options::default();
        assert_eq!(o.wrap_mode, WrapMode::NoWrap);
    }
}
