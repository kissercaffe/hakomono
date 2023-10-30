use crate::errors::Errcode;
use crate::ipc::generate_socketpair;
use crate::hostname::generate_hostname;

use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;



#[derive(Clone)]
pub struct ContainerOpts{
    pub path: CString,
    pub argv: Vec<CString>,

    pub hostname: String,
    pub fd: RawFd,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub addpaths: Vec<(PathBuf, PathBuf)>,
}


impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf, addpaths: Vec<(PathBuf, PathBuf)>) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let argv: Vec<CString> = command.split_ascii_whitespace().map(
            |s| CString::new(s).expect("Cannot read arg")
        ).collect();
        
        let path = argv[0].clone();
        let sockets = generate_socketpair()?;
    
        Ok ((
            ContainerOpts {
                path,
                argv,
                hostname: generate_hostname()?,
                fd: sockets.1.clone(),
                uid,
                mount_dir,
                addpaths,
            },
            sockets
        ))
    }
}