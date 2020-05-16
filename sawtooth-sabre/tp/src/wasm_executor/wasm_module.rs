// Copyright 2018 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sawtooth_sdk::processor::handler::TransactionContext;
use wasmi::{ImportsBuilder, Module, ModuleInstance, RuntimeValue};

use crate::wasm_executor::wasm_externals::{ExternalsError, WasmExternals};

pub struct WasmModule<'a> {
    context: &'a mut dyn TransactionContext,
    module: Module,
}

impl<'a> WasmModule<'a> {
    pub fn new(
        wasm: &[u8],
        context: &'a mut dyn TransactionContext,
    ) -> Result<WasmModule<'a>, ExternalsError> {
        let module = Module::from_buffer(wasm)?;
        Ok(WasmModule { context, module })
    }

    pub fn entrypoint(
        &mut self,
        payload: Vec<u8>,
        signer: String,
        signature: String,
    ) -> Result<Option<i32>, ExternalsError> {
        let mut env = WasmExternals::new(None, self.context)?;

        let instance = ModuleInstance::new(
            &self.module,
            &ImportsBuilder::new().with_resolver("env", &env),
        )?
        .assert_no_start();

        let payload_ptr = env.write_data(payload)? as i32;
        info!("Payload written to memory");

        let signer_ptr = env.write_data(signer.into_bytes())? as i32;
        info!("Signer written to memory");

        let signature_ptr = env.write_data(signature.into_bytes())? as i32;
        info!("Signature written to memory");

        let result = instance.invoke_export(
            "entrypoint",
            &[
                RuntimeValue::I32(payload_ptr),
                RuntimeValue::I32(signer_ptr),
                RuntimeValue::I32(signature_ptr),
            ],
            &mut env,
        )?;

        if let Some(RuntimeValue::I32(i)) = result {
            Ok(Some(i))
        } else {
            Ok(None)
        }
    }
}
