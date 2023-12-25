use std::time::Duration;

use async_timers::OneshotTimer;
use dioxus::prelude::*;

pub fn use_interval(cx: &ScopeState, duration: Duration) -> &UseFuture<()> {
    use_future(cx, (), |_| {
        let update = cx.schedule_update();
        async move {
            loop {
                let mut timer = OneshotTimer::scheduled(duration);
                timer.tick().await;
                update();
            }
        }
    })
}
