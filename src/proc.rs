//! 进程管理模块
//!
//!
//! 支持以下的请求
//! - process-all : 查询指定服务状态
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"process-all"
//! }
//! ```

use jsonrpsee::ws_server::{RpcModule};

use procfs;
use serde::{Deserialize, Serialize};
use hmir_hash::HashWrap;
use std::string;
use std::error::Error;
use nix::sys::signal;
use nix::unistd;
extern crate core_affinity;

#[derive(Clone, Debug,Serialize)]
struct ProcInfo {
    pub pid: i32,
    pub comm: String,
    pub ppid: i32,
    pub vsize: u64,
    pub num_threads: i64
}

pub fn process_all() -> std::string::String
{
    let mut map = HashWrap::<i32,ProcInfo>:: new();
    for prc in procfs::process::all_processes().unwrap() {
        // println!("{:?}",prc);
        if let Ok(stat) = prc.unwrap().stat() {
            // total_time is in seconds
            let p  = ProcInfo {
                pid: stat.pid,
                comm: stat.comm,
                ppid: stat.ppid,
                vsize: stat.vsize,
                num_threads: stat.num_threads
            };
            map.insert(stat.pid,p);
        }
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_all_it_works() {
        let s = process_all();
        println!("{}",s);
    }

    #[test]
    fn process_status_it_works(){
        let s = process_status(0);
        println!("{}",s);
    }
}

pub fn process_status(pid : i32) -> std::string::String {
    if is_valid_process(pid) {

        let process = procfs::process::Process::new(pid);
        let stat = process.unwrap().stat().unwrap();
        let p  = ProcInfo {
            pid: stat.pid,
            comm: stat.comm,
            ppid: stat.ppid,
            vsize: stat.vsize,
            num_threads: stat.num_threads
        };
        let mut map = HashWrap::<i32,ProcInfo>:: new();
        map.insert(stat.pid,p);
        let serialized = serde_json::to_string(&map).unwrap();
        return serialized;
    }
    return string::String::from("Invalid process");
}


fn is_valid_process(pid : i32) -> bool {
    let process = procfs::process::Process::new(pid);
    match process {
        Err(e) => {
            return false;
        },
        _ => {
            return true;
        }
    }
}

pub fn process_kill(pid : i32) -> std::string::String {
    if is_valid_process(pid) {
        signal::kill(unistd::Pid::from_raw(pid), signal::Signal::SIGTERM).unwrap();
        return string::String::from("Ok");
    }
    return string::String::from("Invalid process");
}

pub fn process_bind_cpu(pid : i32) -> std::string::String {
    if is_valid_process(pid) {
        let core_ids = core_affinity::get_core_ids().unwrap();
        let core_id = core_ids[0];
        core_affinity::set_for_current(core_id);
        return string::String::from("Ok");
    }
    return string::String::from("Invalid process");
}


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("process-all", |_, _| {
        //默认没有error就是成功的
        Ok(process_all())
    })?;

    module.register_method("process-status", |params, _| {
        //默认没有error就是成功的
        let pid = params.one::<i32>()?;
        Ok(process_status(pid))
    })?;

    module.register_method("process-kill", |params, _| {
        //默认没有error就是成功的
        let pid = params.one::<i32>()?;
        Ok(process_kill(pid))
    })?;

    module.register_method("process-bind", |params, _| {
        //默认没有error就是成功的
        let pid = params.one::<i32>()?;
        Ok(process_bind_cpu(pid))
    })?;


    Ok(())
}