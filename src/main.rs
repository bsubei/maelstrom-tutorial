mod broadcast;
mod node;
mod protocol;

use broadcast::run_broadcast_server;

fn main() {
    run_broadcast_server();
}
