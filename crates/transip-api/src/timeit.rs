#[macro_export]
macro_rules! timeit {
    ($code:block) => {{
        let start = std::time::Instant::now();
        let t = $code;
        if t.is_err() {
            tracing::error!("error after {} milliseconds", start.elapsed().as_millis());
        } else {
            tracing::info!("result after {} milliseconds", start.elapsed().as_millis());
        };
        t
    }};
}

pub use timeit;
