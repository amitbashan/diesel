use dioxus::prelude::*;

use crate::configuration::Configuration;

pub fn with_mut<T: 'static, O>(
    cx: &ScopeState,
    state: &UseSharedState<T>,
    mutable_callback: impl FnOnce(&mut T) -> O,
) -> O {
    let result = state.with_mut(mutable_callback);
    Configuration::save(cx);
    result
}
