use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};

use crate::{
    bindings_function_http::{self, FunctionHttp},
    domain,
};

use super::{ComponentState, ComponentStateBuilder};

pub(crate) struct FunctionHttpBuilder<'a> {
    state_builder: ComponentStateBuilder,
    component: Component,
    linker: Linker<ComponentState>,
    engine: &'a Engine,
}

impl<'a> FunctionHttpBuilder<'a> {
    pub fn from_binary(engine: &'a Engine, bytes: &[u8]) -> Self {
        let state_builder = ComponentStateBuilder::new();

        let component = Component::from_binary(engine, bytes).expect("Failed to create component");

        let mut linker: Linker<ComponentState> = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI to linker");
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .expect("Failed to add WASI HTTP to linker");

        Self {
            state_builder,
            component,
            linker,
            engine,
        }
    }

    pub fn with_variables(mut self, vars: &[domain::variable::Variable]) -> Self {
        let vars = vars
            .iter()
            .map(|v| (format!("VAR_{}", v.name), &v.value))
            .collect::<Vec<(String, &String)>>();
        self.state_builder.with_envs(&vars);
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.component
            .serialize()
            .expect("Failed to serialize component")
    }

    pub unsafe fn deserialize(engine: &'a Engine, bytes: &[u8]) -> Self {
        let state_builder = ComponentStateBuilder::new();

        let component =
            Component::deserialize(engine, bytes).expect("Failed to deserialize component");

        let mut linker: Linker<ComponentState> = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI to linker");
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .expect("Failed to add WASI HTTP to linker");

        Self {
            state_builder,
            component,
            linker,
            engine,
        }
    }

    pub async fn build(self) -> (FunctionHttp, Store<ComponentState>) {
        let component_state = self.state_builder.build();
        let mut store = Store::new(self.engine, component_state);

        let func_instance = bindings_function_http::FunctionHttp::instantiate_async(
            &mut store,
            &self.component,
            &self.linker,
        )
        .await
        .expect("Failed to instantiate component");
        (func_instance, store)
    }
}
