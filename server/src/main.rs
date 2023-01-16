mod events;

use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        
        .add_startup_system(setup)
        .add_plugin(events::event_processor::EventProcessorPlugin)
        .run();

}

fn setup() {}


