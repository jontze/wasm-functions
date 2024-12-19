use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::bindings_function_http::{self, FunctionHttp};

pub(crate) struct ComponentState {
    ctx: WasiCtx,
    http_ctx: wasmtime_wasi_http::WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for ComponentState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl wasmtime_wasi_http::WasiHttpView for ComponentState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub(crate) struct FunctionHttpBuilder {
    pub store: Store<ComponentState>,
    component: Component,
    linker: Linker<ComponentState>,
}

impl FunctionHttpBuilder {
    pub fn from_binary(engine: &Engine, bytes: &[u8]) -> Self {
        let res_table = ResourceTable::new();
        let wasi_ctx = WasiCtxBuilder::new().inherit_env().build();
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
        let wasi_ctx = WasiCtxBuilder::new().inherit_env().build();
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

    pub async fn build(&mut self) -> FunctionHttp {
        bindings_function_http::FunctionHttp::instantiate_async(
            &mut self.store,
            &self.component,
            &self.linker,
        )
        .await
        .expect("Failed to instantiate module")
    }
}
