use crate::layer::MetricsExt;
use tracing::{span, Level};

#[derive(Default, Debug)]
pub struct Thingy;

impl Thingy {
    pub fn handle_unshaved(&self, yak: usize) {
        let span = span!(Level::INFO, "handle_unshaved", yak);
        // let span = Span::current();
        span.with_timer();
        let _span = span.enter();
    }
}
