use metrics::{counter, timing};
use quanta::Clock;
use std::fmt::Debug;
use tracing_core::{
    span::{Attributes, Id, Record},
    Event, Metadata, Subscriber,
};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::{LookupSpan, Registry},
};

#[derive(Default)]
pub struct Metrics {
    clock: Clock,
}

pub trait MetricsExt {
    fn with_timer(&self);
}

impl MetricsExt for tracing::Span {
    fn with_timer(&self) {
        self.with_subscriber(|(id, subscriber)| {
            if let Some(_) = subscriber.downcast_ref::<Metrics>() {
                if let Some(registry) = subscriber.downcast_ref::<Registry>() {
                    let span = registry
                        .span(id)
                        .expect("in new_span but span does not exist");
                    let data = MetricData::default();
                    span.extensions_mut().replace(data);
                }
            }
        });
    }
}

#[derive(Default)]
struct MetricData {
    enter_count: u64,
    entered: Option<u64>,
    exited: Option<u64>,
}

impl MetricData {
    pub fn mark_entered(&mut self, now: u64) {
        self.enter_count += 1;
        if self.entered.is_none() {
            self.entered.replace(now);
        }
    }

    pub fn mark_exited(&mut self, now: u64) {
        self.exited.replace(now);
    }

    pub fn flush(&mut self, metadata: &'static Metadata<'static>) {
        if self.enter_count > 0 {
            counter!(format!("{}", metadata.name()), self.enter_count);
            timing!(
                format!("{}_ns", metadata.name()),
                self.entered.take().unwrap(),
                self.exited.take().unwrap()
            );
        }
    }
}

impl<S> Layer<S> for Metrics
where
    S: Subscriber + for<'span> LookupSpan<'span> + Debug,
{
    fn new_span(&self, _: &Attributes, id: &Id, ctx: Context<S>) {
        // let data = MetricData::default();
        // let span = ctx.span(id).expect("in new_span but span does not exist");
        // span.extensions_mut().insert(data);
    }

    fn on_record(&self, _: &Id, _: &Record<'_>, _: Context<S>) {}

    fn on_event(&self, _: &Event<'_>, _: Context<S>) {}

    fn on_enter(&self, id: &Id, ctx: Context<S>) {
        let span = ctx.span(id).expect("in on_enter but span does not exist");
        let mut ext = span.extensions_mut();
        if let Some(data) = ext.get_mut::<MetricData>() {
            data.mark_entered(self.clock.now());
        }
    }

    fn on_exit(&self, id: &Id, ctx: Context<S>) {
        let now = self.clock.now();
        let span = ctx.span(id).expect("in on_exit but span does not exist");
        let mut ext = span.extensions_mut();
        if let Some(data) = ext.get_mut::<MetricData>() {
            data.mark_exited(now);
        }
    }

    fn on_close(&self, id: Id, ctx: Context<S>) {
        let span = ctx.span(&id).expect("in on_close but span does not exist");
        let mut ext = span.extensions_mut();
        if let Some(data) = ext.get_mut::<MetricData>() {
            data.flush(span.metadata());
        }
    }
}
