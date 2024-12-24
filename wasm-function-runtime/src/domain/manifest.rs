use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Manifest {
    pub function: Function,
    pub http: Option<HttpFunc>,
    pub scheduled: Option<ScheduledFunc>,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) enum FuncKind {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "scheduled")]
    Scheduled,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct Function {
    pub name: String,
    pub trigger: FuncKind,
    pub scope: String,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) enum HttpFuncMehod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}

impl ToString for HttpFuncMehod {
    fn to_string(&self) -> String {
        match self {
            HttpFuncMehod::Get => "GET".to_string(),
            HttpFuncMehod::Post => "POST".to_string(),
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct HttpFunc {
    pub path: String,
    pub method: HttpFuncMehod,
    pub public: bool,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct ScheduledFunc {
    pub cron: String,
}

#[cfg(test)]
mod tests_http_manifest {
    use super::*;

    #[test]
    fn parse_full_http_manifest() {
        let toml_http_function_manifest = r#"
            [function]
            name = "my-http-function"
            scope = "my-scope"
            trigger = "http"

            [http]
            path = "/my-http-function"
            method = "GET"
            public = true
        "#;

        let manifest: Manifest = toml::from_str(toml_http_function_manifest).unwrap();

        assert_eq!(manifest.function.name, "my-http-function");
        assert_eq!(manifest.function.scope, "my-scope");
        assert_eq!(manifest.function.trigger, FuncKind::Http);
        assert_eq!(manifest.http.as_ref().unwrap().path, "/my-http-function");
        assert_eq!(manifest.http.as_ref().unwrap().method, HttpFuncMehod::Get);
        assert_eq!(manifest.http.as_ref().unwrap().public, true);
    }
}

#[cfg(test)]
mod tests_scheduled_manifest {
    use super::*;

    #[test]
    fn parse_full_scheduled_manifest() {
        let toml_scheduled_function_manifest = r#"
            [function]
            name = "my-scheduled-function"
            scope = "my-scope"
            trigger = "scheduled"

            [scheduled]
            cron = "0 0 * * *"
        "#;

        let manifest: Manifest = toml::from_str(toml_scheduled_function_manifest).unwrap();

        assert_eq!(manifest.function.name, "my-scheduled-function");
        assert_eq!(manifest.function.scope, "my-scope");
        assert_eq!(manifest.function.trigger, FuncKind::Scheduled);
        assert_eq!(manifest.scheduled.as_ref().unwrap().cron, "0 0 * * *");
    }
}
