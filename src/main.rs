use metrics::{Key, Recorder};
use tracing::{debug, error, info, span, trace, warn, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

mod thingy;
use self::thingy::Thingy;

mod layer;
use self::layer::{Metrics, MetricsExt};

#[derive(Default)]
struct PrintRecorder;

fn nanos_to_readable(t: u64) -> String {
    let f = t as f64;
    if f < 1_000.0 {
        format!("{}ns", f)
    } else if f < 1_000_000.0 {
        format!("{:.0}μs", f / 1_000.0)
    } else if f < 2_000_000_000.0 {
        format!("{:.2}ms", f / 1_000_000.0)
    } else {
        format!("{:.3}s", f / 1_000_000_000.0)
    }
}

impl Recorder for PrintRecorder {
    fn increment_counter(&self, key: Key, value: u64) {
        // println!("metrics -> counter(name={}, value={})", key, value);
    }

    fn update_gauge(&self, key: Key, value: i64) {
        // println!("metrics -> gauge(name={}, value={})", key, value);
    }

    fn record_histogram(&self, key: Key, value: u64) {
        let value = nanos_to_readable(value);
        println!("metrics -> histogram(name={}, value={})", key, value);
    }
}

#[tracing::instrument]
fn shave(yak: usize) -> bool {
    dbg!(Span::current());
    Span::current().with_timer();
    // let span = span!(Level::INFO, "shave", yak);
    // let _span = span.enter();
    debug!(
        message = "hello! I'm gonna shave a yak.",
        excitement = "yay!"
    );
    if yak == 3 {
        warn!(target: "yak_events", "could not locate yak!");
        false
    } else {
        trace!(target: "yak_events", "yak shaved successfully");
        true
    }
}

fn shave_all(yaks: usize) -> usize {
    let span = span!(Level::TRACE, "shaving_yaks", yaks_to_shave = yaks);
    span.with_timer();
    let _enter = span.enter();

    info!("shaving yaks");

    let mut num_shaved = 0;
    for yak in 1..=yaks {
        let shaved = shave(yak);
        trace!(target: "yak_events", yak, shaved);

        if !shaved {
            let thingy = Thingy::default();
            thingy.handle_unshaved(yak);
            error!(message = "failed to shave yak!", yak);
        } else {
            num_shaved += 1;
        }

        trace!(target: "yak_events", yaks_shaved = num_shaved);
    }

    num_shaved
}

fn main() {
    let recorder = PrintRecorder::default();
    metrics::set_boxed_recorder(Box::new(recorder)).unwrap();

    let subscriber = Registry::default().with(Metrics::default());
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let number_of_yaks = 3;
    debug!("preparing to shave {} yaks", number_of_yaks);

    let number_shaved = shave_all(number_of_yaks);

    debug!(
        message = "yak shaving completed.",
        all_yaks_shaved = number_shaved == number_of_yaks,
    );
}
