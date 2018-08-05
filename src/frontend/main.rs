extern crate yew;
extern crate wakkave;

use yew::prelude::*;
use wakkave::frontend::components::root::RootComponent;

fn main() {
    yew::initialize();
    App::<RootComponent>::new().mount_to_body();
    yew::run_loop();
}
