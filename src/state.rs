use crate::css::Theme;

#[derive(Debug, Clone, Default)]
pub(crate) struct GlobalState {
    pub(crate) theme: Theme,
}
