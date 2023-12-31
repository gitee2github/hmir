//! 用于启动终端
//!
//!

use jsonrpsee::ws_server::{RpcModule};
use std::process::{Command,Stdio};
use std::sync::{Arc, Mutex};
use nix::sys::signal;
use nix::unistd;
use hmir_hash::HashWrap;
use std::thread;

use log::{info};

type Pid = Arc<Mutex<u32>>;

use hmir_errno::errno;
use hmir_token::TokenChecker;


lazy_static! {
    static ref TTY_ID : Pid = {
        Arc::new(Mutex::new(0))
    };
}


macro_rules! ttyd_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.error($i,String::from(errno::HMIR_MSG[$i]));
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn ttyd_start() -> String
{
    return futures::executor::block_on(aysnc_ttyd_start());
    // return ttyd_start_test();
}



pub async fn aysnc_ttyd_start() -> String
{

    if *TTY_ID.lock().unwrap() != 0 {
        ttyd_default_result!(0);
    } else {
        let (tx, rx) = std::sync::mpsc::channel();
        let _thread_join_handle = thread::spawn(move || {
            info!("The ttyd has start its execution !");
            if let Ok(mut child) = Command::new("ttyd")
                .arg("-p").arg("3001")
                .arg("-u").arg("0")
                .arg("-g").arg("0")
                .arg("-w").arg("/root")
                .arg("/bin/login")
                .stdout(Stdio::null())
                .spawn()
            {
                *TTY_ID.lock().unwrap() = child.id();
                let _rt = tokio::runtime::Runtime::new().unwrap();
                tx.send("true").unwrap();
                child.wait().expect("command wasn't running");
                info!("The ttyd has finished its execution!");
            }
        });

        let r = rx.recv_timeout(std::time::Duration::from_millis(500));
        match r {
            Ok(msg) => {
                if msg == "true" {
                    ttyd_default_result!(0);
                }else {
                    ttyd_default_result!(errno::HMIR_ERR_COMM);
                }
            }
            _ => {
                ttyd_default_result!(errno::HMIR_ERR_COMM);
            }
        }
    }
}



pub fn ttyd_stop() -> String
{
    if *TTY_ID.lock().unwrap() != 0 {
        let id = *TTY_ID.lock().unwrap() as i32;
        signal::kill(unistd::Pid::from_raw(id), signal::Signal::SIGHUP).unwrap();
        *TTY_ID.lock().unwrap() = 0;
    }
    ttyd_default_result!(0);
}




pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("ttyd-start", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);


        Ok(ttyd_start())
    })?;

    module.register_method("ttyd-stop", |params, _| {
        //默认没有error就是成功的

        /*
               {
                    "jsonrpc":"2.0",
                    "id":1,
                    "method":"ttyd-stop",
                    "params":["aaaaaaa"]
               }

               {
                    "jsonrpc":"2.0",
                    "id":1,
                    "method":"ttyd-stop"
               }
         */
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        
        Ok(ttyd_stop())


    })?;

    Ok(())

}