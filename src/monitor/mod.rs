use std::time::Duration;

use apalis::prelude::{BoxDynError, Error, Event, Monitor};
use tracing::{error, info};

pub trait OrApalisError<T> {
    fn or_apalis_error(self) -> Result<T, Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> OrApalisError<T> for Result<T, E> {
    fn or_apalis_error(self) -> Result<T, Error> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(apalis::prelude::Error::from(Box::new(err) as BoxDynError)),
        }
    }
}

pub trait MonitorExt {
    fn setup() -> Self;
}

impl MonitorExt for Monitor {
    fn setup() -> Self {
        Monitor::new()
            .on_event(|e| {
                let worker_id = e.id();
                match e.inner() {
                    Event::Engage(task_id) => {
                        info!("Worker [{worker_id}] got a job with id: {task_id}");
                    }
                    Event::Error(e) => {
                        error!("Worker [{worker_id}] encountered an error: {e}");
                    }

                    Event::Exit => {
                        info!("Worker [{worker_id}] exited");
                    }
                    Event::Idle => {
                        info!("Worker [{worker_id}] is idle");
                    }
                    Event::Start => {
                        info!("Worker [{worker_id}] started");
                    }
                    Event::Stop => {
                        info!("Worker [{worker_id}] stopped");
                    }
                    _ => {}
                }
            })
            .shutdown_timeout(Duration::from_millis(5000))
    }
}
