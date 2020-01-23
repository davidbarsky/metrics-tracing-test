use crate::layer::MetricsExt;
use tracing::Span;

#[derive(Default, Debug)]
pub struct Thingy;

impl Thingy {
    #[tracing::instrument]
    pub fn handle_unshaved(&self, yak: usize) {
        Span::current().with_timer();
    }
}
