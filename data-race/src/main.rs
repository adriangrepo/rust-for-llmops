mod channels;
use channels::channel_main;
mod arc;
use arc::arc_main;

fn main() {
    channel_main();
    arc_main();
}
