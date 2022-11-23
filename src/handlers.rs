use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use warp::{self};

use std::path::Path;



use thiserror::Error;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    dock::{Param, VmDock},
    Module, Vm,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Results {
    pub message: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
   pub name: String,
}



pub async fn get_wasm(item: Item) -> Result<Box<dyn warp::Reply>, Infallible> {


    let message = match load_wasm(item.name) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };


    let result = Results {
        message: message,
    };


    Ok(Box::new(warp::reply::json(&result)))

}



pub fn load_wasm(name: String) -> Result<String, Box<dyn std::error::Error>> {

    let wasm_file = Path::new("./helloazurefuncwasm.wasm");
    let module = Module::from_file(None, wasm_file)?;

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    let vm = Vm::new(Some(config))?.register_module(None, module)?;

    let vm = VmDock::new(vm);

    // call func "say_ok" in wasm-lib.wasm: String -> Result<(u16, String), String>

    let params = vec![Param::String(&name)];
    match vm
        .run_func("say_ok", params)
        .map_err(|e| WasmError::RunHostFunc(e.to_string()))?
    {
        Ok(mut res) => {
            let val = res.pop().unwrap().downcast::<String>().unwrap();
            Ok(*val)
        }

        Err(err) => Err(Box::new(WasmError::HostFunc(err))),
    }


}



#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum WasmError {
    #[error("{0}")]
    Operation(String),
    #[error("{0}")]
    LoadWasm(String),
    #[error("{0}")]
    CreateConfig(String),
    #[error("{0}")]
    CreateVm(String),
    #[error("{0}")]
    RegisterModule(String),
    #[error("{0}")]
    RunHostFunc(String),
    #[error("{0}")]
    HostFunc(String),
}