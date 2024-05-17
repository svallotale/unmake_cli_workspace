use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub fn start_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {msg}").unwrap()
        .progress_chars("#>-"));
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

