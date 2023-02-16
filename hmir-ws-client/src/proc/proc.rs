use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use crate::ws_client::RequestClient;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;
use hmir_protocol::proc;


impl RequestClient {
    #[allow(dead_code)]
    pub fn proc_process_info(&self) -> (usize, String) {

        let token = self.token.clone();
        let (state,info) = self.runtime.block_on(async{
            let response: Result<String, _> = self.client.request("process-all", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,proc::ProcInfo> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return (state,info);
    }
}