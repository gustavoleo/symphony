use std::env;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct ServiceConfig {
    poll_interval_secs: u64,
    workspace_root: String,
    tracker_kind: String,
}

impl ServiceConfig {
    fn from_env() -> Self {
        Self {
            poll_interval_secs: env::var("SYMPHONY_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(30),
            workspace_root: env::var("SYMPHONY_WORKSPACE_ROOT")
                .unwrap_or_else(|_| "./workspaces".to_string()),
            tracker_kind: env::var("SYMPHONY_TRACKER_KIND")
                .unwrap_or_else(|_| "linear".to_string()),
        }
    }
}

fn main() {
    let cfg = ServiceConfig::from_env();

    println!("Starting Symphony (Rust preview)");
    println!("config: {:?}", cfg);

    loop {
        println!(
            "tick: polling tracker={} with workspace_root={}",
            cfg.tracker_kind, cfg.workspace_root
        );
        thread::sleep(Duration::from_secs(cfg.poll_interval_secs));
    }
}
