// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[macro_use]
extern crate log;

use anyhow::Result;
use teaclave_binder::proto::{ECallCommand, StartServiceInput, StartServiceOutput};
use teaclave_binder::TeeBinder;
use teaclave_types::TeeServiceResult;

fn main() -> Result<()> {
    env_logger::init();
    let tee = TeeBinder::new(env!("CARGO_PKG_NAME"))?;
    run(&tee)?;

    Ok(())
}

fn start_enclave_service(tee: &TeeBinder) -> Result<()> {
    info!("Start enclave service");
    let config = teaclave_config::RuntimeConfig::from_toml("runtime.config.toml")?;
    let input = StartServiceInput { config };
    let cmd = ECallCommand::StartService;
    match tee.invoke::<StartServiceInput, TeeServiceResult<StartServiceOutput>>(cmd, input) {
        Err(e) => error!("{:?}", e),
        Ok(Err(e)) => error!("{:?}", e),
        _ => (),
    }

    Ok(())
}

fn run(tee: &TeeBinder) -> Result<()> {
    start_enclave_service(tee)?;

    Ok(())
}
