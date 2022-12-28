//! 用于启动终端
//!
//!

use jsonrpsee::ws_server::{RpcModule};
use std::process::{Command,Stdio};
use std::sync::{Arc, Mutex};
use nix::sys::signal;
use nix::unistd;
use hmir_hash::HashWrap;
use tokio::sync::mpsc;

use log::{error,info};

type Pid = Arc<Mutex<u32>>;

lazy_static! {
    static ref tty_id : Pid = {
        Arc::new(Mutex::new(0))
    };
}


macro_rules! ttyd_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn ttyd_start() -> String
{
    // return futures::executor::block_on(_ttyd_start());
    return org_ttyd_start();
}


pub fn org_ttyd_start() -> String
{
    if *tty_id.lock().unwrap() != 0 {
        ttyd_default_result!(0);
    } else {
        tokio::task::spawn(async move {
            //运行在一个不阻塞的线程
            info!("The ttyd has start its execution !");
            if let Ok(mut child) = Command::new("ttyd").arg("-p").arg("5899").arg("bash")
                .stdout(Stdio::null())
                .spawn()
            {
                println!("lock tx.send called");
                *tty_id.lock().unwrap() = child.id();
                println!("before tx.send called");
                println!("tx.send called");
                child.wait().expect("command wasn't running");
                info!("The ttyd has finished its execution!");
            }
        });
        ttyd_default_result!(0);
    }
}

pub fn aysnc_ttyd_start() -> String
{
    if *tty_id.lock().unwrap() != 0 {
        ttyd_default_result!(0);
    } else {
        let (tx, mut rx) = mpsc::channel(32);
        tokio::task::spawn(async move {
            //运行在一个不阻塞的线程
            info!("The ttyd has start its execution !");
            if let Ok(mut child) = Command::new("ttyd").arg("-p").arg("5899").arg("bash")
                .stdout(Stdio::null())
                .spawn()
            {
                println!("lock tx.send called");
                *tty_id.lock().unwrap() = child.id();
                println!("before tx.send called");
                tx.send("sending from first handle").await;
                println!("tx.send called");
                child.wait().expect("command wasn't running");
                info!("The ttyd has finished its execution!");
            }
        });

        println!("Before select ");
        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
        println!("After select ");

        ttyd_default_result!(0);
    }
}

pub fn ttyd_stop() -> String
{
    if ( *tty_id.lock().unwrap() != 0 ){
        let id = *tty_id.lock().unwrap() as i32;
        signal::kill(unistd::Pid::from_raw(id), signal::Signal::SIGHUP).unwrap();
        *tty_id.lock().unwrap() = 0;
    }
    ttyd_default_result!(0);
}

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("ttyd-start", |_, _| {
        //默认没有error就是成功的
        Ok(ttyd_start())
    })?;

    module.register_method("ttyd-stop", |_, _| {
        //默认没有error就是成功的
        Ok(ttyd_stop())
    })?;

    Ok(())

}