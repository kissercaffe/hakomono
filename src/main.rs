#[macro_use] extern crate scan_fmt;

mod capabilities;
mod child;
mod cli;
mod container;
mod config;
mod errors;
mod hostname;
mod ipc;
mod mounts;
mod namespaces;
mod resources;
mod syscalls;


use crate::errors::exit_with_retcode;
use std::process::exit;


fn main() {
    match cli::parse_args() {
        Ok(args) =>{
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsion arguments:\n\t{}", e);
            exit(e.get_retcode());
        }

    }
}