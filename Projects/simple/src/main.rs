trait Plugin {
    fn run(&self);
}

struct LoggerPlugin;
struct AnalyticsPlugin;

impl Plugin for LoggerPlugin {
    fn run(&self) {
        println!("LoggerPlugin is running: Logging system initialized.");
    }
}

impl Plugin for AnalyticsPlugin {
    fn run(&self) {
        println!("AnalyticsPlugin is running: Analytics data collected.");
    }
}

fn main() -> anyhow::Result<()> {
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
    plugins.push(Box::new(LoggerPlugin));
    plugins.push(Box::new(AnalyticsPlugin));
    Ok(())
}