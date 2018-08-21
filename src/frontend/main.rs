extern crate wakkave;
extern crate yew;

use wakkave::frontend::components::root::RootComponent;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<RootComponent>::new().mount_to_body();
    yew::run_loop();
}
