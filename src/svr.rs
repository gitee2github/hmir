
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use std::sync::RwLock;
use hmir_hash::HashWrap;
use std::{thread, time};


use hmir_systemd::{
    build_blocking_client,
    manager::blocking::{OrgFreedesktopSystemd1Manager},
    models::{Unit,IntoModel},
    SystemdObjectType,
};


lazy_static! {
    static ref SERVICE_MAP : RwLock<HashWrap<Unit>> = {
        let m  = HashWrap::<Unit>:: new();
        RwLock::new(m)
    };
}


#[doc(hidden)]
fn update_all_svr()
{
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let units = client.list_units().unwrap();
    for unit in units {
        let unit: Unit = unit.into_model().unwrap().clone();
        SERVICE_MAP.write().unwrap().insert(unit.name.clone(),unit.clone());
    }
}

#[doc(hidden)]
fn create_svrmg_thread()
{
    tokio::task::spawn(async{
        //运行在一个不阻塞的线程
        let sec = time::Duration::from_millis(1000);
        loop {
            update_all_svr();
            thread::sleep(sec);
        }
    });
}


#[doc(hidden)]
pub fn init_services_mg()  {
    create_svrmg_thread();
}


///
/// service-all接口实现
///
/// 获取所有服务信息
pub fn service_all() -> String {
    let serialized = serde_json::to_string(&*SERVICE_MAP).unwrap();
    serialized
}

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("service-all", |_, _| {
        //默认没有error就是成功的
        Ok(service_all())
    })?;


    Ok(())
}