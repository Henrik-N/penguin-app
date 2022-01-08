# Penguin App

Penguin app is an appbuilder and abstraction layer for looping applications built on [winit](https://github.com/rust-windowing/winit), [legion ecs](https://github.com/amethyst/legion) and [fern logger](https://github.com/daboross/fern). It also adds a [bevy](https://github.com/bevyengine/bevy)-like plugin implementation to legion.

The package also includes an optional feature adding a time resource plugin, enabling easy access to deltatime.


## Dependencies
#### Cargo.toml
```toml
[dependencies]
penguin-application = { version = "0.1", features = ["time-plugin"]}
penguin-config = { version = "0.1" }
```

## Configuration
Create an app-config.json file like this and put in your project root directory. Available logger levels are ["error", "warning", "info", "debug", "trace"] in decending order of importance. 

#### app-config.json

```json
{
  "logger_config": {
    "output_path": "logs/output.log",
    "debug_message_severity": "debug"
  },
  "window_config": {
    "width": 640,
    "height": 400
  }
}
```


## Usage
### Log
Log messages are generated with the standard [log](https://github.com/rust-lang/log) function calls, such as ```log::error!("message")```.
### App
```rust
use penguin_config::PenguinConfig;
use penguin_app::{App, config::AppConfig};

fn main() {
    App::builder(AppConfig::read_config())
        .add_plugin(penguin_app::time_plugin::TimePlugin)
        .run()
        .unwrap();
}
```
### Plugin
```rust
use penguin_app::ecs::*;
use penguin_app::time_plugin::Time;


pub struct MeasureTimePlugin;

impl Plugin for MeasureTimePlugin {
    fn startup(&mut self, resources: &mut Resources) -> Vec<Step> {
        resources.insert(PassedTimeResource::default());
        vec![]
    }

    fn run() -> Vec<Step> {
        Schedule::builder()
            .add_system(measure_time_system())
            .build()
            .into_vec()
    }

    fn shutdown() -> Vec<Step> {
        vec![] 
    }
}


#[derive(Default)]
struct PassedTimeResource(f32);

#[system]
fn measure_time(#[resource] passed_time: &mut PassedTimeResource, #[resource] time: &Time) {
    passed_time.0 += time.delta();
    log::info!("Time passed: {}", passed_time.0);
}
```
