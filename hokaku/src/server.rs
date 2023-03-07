use std::{thread, sync::Arc};

use obs_wrapper::prelude::LoadContext;

use crate::HokakuModule;

pub(crate) fn start_server(module: &mut HokakuModule, ctx: &LoadContext) {
    // something
    thread::spawn(server_loop);
}

fn server_loop() {

}