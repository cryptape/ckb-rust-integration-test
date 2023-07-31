use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
use std::fmt::format;

pub use expect::get_state;

pub async fn mock_rpc() -> Result<MockData, Box<dyn std::error::Error>> {
    //todo find framework support such as js(mocha or jest)
    let data = get_method_and_params(get_state().current_test_name().to_string());
    get_mock_test_data(&data.method, &data.params).await
}

pub async fn get_mock_test_data(method: &str, params: &str) -> Result<MockData, Box<dyn std::error::Error>> {
    let url = format!("http://127.0.0.1:5000/test/{}/{}", method, params);
    println!("{}", url);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    // 检查 HTTP 响应状态码
    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status code: {}", response.status()).into());
    }

    let response_text = response.text().await?;
    let data: serde_json::Value = serde_json::from_str(&response_text)?;


    let request_data = data.get("request").cloned().unwrap_or_default();
    let response_data = data.get("response").cloned().unwrap_or_default();
    let rpc_client = RPCClient::new(&url);
    Ok(MockData {
        rpc_client,
        request_data,
        response_data,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockData {
    pub(crate) rpc_client: RPCClient,
    request_data: serde_json::Value,
    pub(crate) response_data: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RPCClient {
    url: String,
}

impl RPCClient {
    fn new(url: &str) -> RPCClient {
        RPCClient {
            url: url.to_string(),
        }
    }
}

fn get_method_and_params(current_test_name: String) -> TestParams {
    let (method, params) = split_first_space(&current_test_name);
    TestParams {
        method:method.to_string(),
        params: params.to_string(), }
}

fn split_first_space(input: &str) -> (&str, &str) {
    if let Some(index) = input.find(' ') {
        let (first_part, rest_part) = input.split_at(index);
        (first_part, rest_part.trim_start())
    } else {
        (input, "")
    }
}

struct TestParams {
    method: String,
    params: String,
}

impl TestParams {
    fn new(method: &str, params: &str) -> TestParams {
        TestParams {
            method: method.to_string(),
            params: params.to_string(),
        }
    }
}

mod expect {
    pub fn get_state() -> TestExpect {
        // todo
        TestExpect
    }

    pub struct TestExpect;
    impl TestExpect {
        pub fn current_test_name(&self) -> &str {
            "test_function_name"
        }
    }
}