extern crate yew;
extern crate wakkave;

use yew::prelude::*;
use wakkave::frontend::login::Login;

fn main() {
    yew::initialize();
    App::<Login>::new().mount_to_body();
    yew::run_loop();
}
