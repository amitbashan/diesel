use std::time::Duration;

use dioxus::prelude::*;
use gloo::timers::future::sleep;

pub fn use_interval(cx: &ScopeState, duration: Duration) -> &UseFuture<()> {
    use_future(cx, (), |_| {
        let update = cx.schedule_update();
        async move {
            loop {
                sleep(duration).await;
                update();
            }
        }
    })
}
