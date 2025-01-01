use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtxBuilder};

use crate::bindings_function_scheduled;

use super::ComponentState;

pub(crate) struct FunctionScheduledBuilder {
    pub store: Store<ComponentState>,
    component: Component,
    linker: Linker<ComponentState>,
}

impl FunctionScheduledBuilder {
    pub fn from_binary(engine: &Engine, bytes: &[u8]) -> Self {
        let res_table = ResourceTable::new();
        let wasi_ctx = WasiCtxBuilder::new().build();
        let wasi_http_ctx = wasmtime_wasi_http::WasiHttpCtx::new();

        let component = Component::from_binary(engine, bytes).expect("Failed to create component");
        let state = ComponentState {
            ctx: wasi_ctx,
            http_ctx: wasi_http_ctx,
            table: res_table,
        };

        let mut linker: Linker<ComponentState> = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI to linker");
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .expect("Failed to add WASI HTTP to linker");

        Self {
            store: Store::new(engine, state),
            component,
            linker,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.component
            .serialize()
            .expect("Failed to serialize component")
    }

    pub unsafe fn deserialize(engine: &Engine, bytes: &[u8]) -> Self {
        let res_table = ResourceTable::new();
        let wasi_ctx = WasiCtxBuilder::new().build();
        let wasi_http_ctx = wasmtime_wasi_http::WasiHttpCtx::new();

        let component =
            Component::deserialize(engine, bytes).expect("Failed to deserialize component");

        let state = ComponentState {
            ctx: wasi_ctx,
            http_ctx: wasi_http_ctx,
            table: res_table,
        };

        let mut linker: Linker<ComponentState> = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI to linker");
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .expect("Failed to add WASI HTTP to linker");

        Self {
            store: Store::new(engine, state),
            component,
            linker,
        }
    }

    pub async fn build(&mut self) -> bindings_function_scheduled::FunctionScheduled {
        bindings_function_scheduled::FunctionScheduled::instantiate_async(
            &mut self.store,
            &self.component,
            &self.linker,
        )
        .await
        .expect("Failed to instantiate module")
    }
}
