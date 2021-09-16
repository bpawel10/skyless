use crate::core::prelude::*;
use std::time::Duration;
use tokio::time;

system! {
    #[event]
    pub struct TickEvent(pub Duration);

    task! {
        let duration = Duration::from_secs(1);
        let mut interval = time::interval(duration);

        loop {
            interval.tick().await;
            yield Some(Arc::new(TickEvent(duration)) as EventType);
        }
    }

    #[effect(TickEvent)]
    fn handle_tick(_: EventType, _: GameAttributesType, _: WorldType) -> EffectResultType {
        println!("Handle tick");
        Some((Vec::new(), Vec::new()))
    }
}
