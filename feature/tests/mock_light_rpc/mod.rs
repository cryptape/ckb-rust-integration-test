mod fetch_header;
mod fetch_transaction;
mod get_cells;
mod get_cells_capacity;
mod get_genesis_block;
mod get_header;
mod get_peers;
mod get_scripts;
mod get_tip_header;
mod get_transaction;
mod get_transactions;
mod local_node_info;
mod send_transaction;
mod set_scripts;

use ckb_sdk::rpc::LightClientRpcClient;
// use ckb_sdk::LightClientRpcClient;
use reqwest::blocking::Client;
use rstest::*;


pub fn get_mock_test_data(method: &str, params: &str) -> Result<MockData, Box<dyn std::error::Error>> {
    let url = format!("http://127.0.0.1:5001/test/{}/{}", method, params);
    let client = Client::new();
    let response = client.get(&url).send()?;

    // 检查 HTTP 响应状态码
    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status code: {}", response.status()).into());
    }

    let response_text = response.text()?;
    let data: serde_json::Value = serde_json::from_str(&response_text)?;
    let request_data = data.get("request").cloned().unwrap_or_default();
    let response_data = data.get("response").cloned().unwrap_or_default();
    Ok(MockData {
        url,
        request_data,
        response_data,
    })
}

#[derive(Debug)]
pub struct MockData {
    pub(crate) url: String,
    pub(crate) request_data: serde_json::Value,
    pub(crate) response_data: serde_json::Value,
}

#[derive(Debug)]
pub struct MockRpcData {
    pub(crate) method: String,
    pub(crate) params: String,
    pub(crate) url: String,
    pub(crate) request_data: serde_json::Value,
    pub(crate) response_data: serde_json::Value,
}

impl MockRpcData {
    pub fn new(method: String, params: String) -> Self {
        let data = get_mock_test_data(method.as_str(), params.as_str()).unwrap();
        MockRpcData {
            method,
            params,
            url: data.url,
            request_data: data.request_data,
            response_data: data.response_data,
        }
    }
    // pub fn default() -> MockRpcData {
    //     let method = "21";
    //     let params = "21";
    //     MockRpcData::new(method.into(), params.into())
    // }

    // pub fn get_mock_data(&self) -> MockData {
    //     let result = get_mock_test_data(self.method.as_str(), self.params.as_str()).unwrap();
    //     result
    // }
    pub fn client(&self) -> LightClientRpcClient {
        let client = LightClientRpcClient::new(self.url.as_str());
        client
    }
}


#[fixture]
pub fn mock_rpc_data(
    #[default = "Alice"] name: impl AsRef<str>,
    #[default = "params"] params: impl AsRef<str>,
) -> MockRpcData {
    return MockRpcData::new(name.as_ref().to_owned(), params.as_ref().to_owned());
}