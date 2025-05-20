mod add_node;
mod calculate_dao_field;
mod calculate_dao_maximum_withdraw;
mod clear_banned_addresses;
mod clear_tx_pool;
mod dry_run_transaction;
mod estimate_cycles;
mod generate_block;
mod generate_block_with_template;
mod get_banned_addresses;
mod get_block;
mod get_block_by_number;
mod get_block_economic_state;
mod get_block_filter;
mod get_block_hash;
mod get_block_median_time;
mod get_block_template;
mod get_blockchain_info;
mod get_cells;
mod get_cells_capacity;
mod get_consensus;
mod get_current_epoch;
mod get_deployments_info;
mod get_epoch_by_number;
mod get_fee_rate_statics;
mod get_fee_rate_statistics;
mod get_header;
mod get_header_by_number;
mod get_indexer_tip;
mod get_live_cell;
mod get_peers;
mod get_raw_tx_pool;
mod get_tip_block_number;
mod get_tip_header;
mod get_transaction;
mod get_transaction_and_witness_proof;
mod get_transaction_proof;
mod get_transactions;
mod local_node_info;
mod ping_peers;
mod remove_node;
mod remove_transaction;
mod send_alert;
mod send_transaction;
mod set_ban;
mod set_network_active;
mod submit_block;
mod sync_state;
mod truncate;
mod tx_pool_info;
mod tx_pool_ready;
mod verify_transaction_and_witness_proof;
mod verify_transaction_proof;
mod test_tx_pool_accept;
mod clear_tx_verify_queue;
mod send_test_transaction;

use ckb_sdk::CkbRpcClient;
use reqwest::blocking::Client;
use rstest::*;


pub fn get_mock_test_data(method: &str, params: &str) -> Result<MockData, Box<dyn std::error::Error>> {
    let url = format!("http://127.0.0.1:5000/test/{}/{}", method, params);
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
    pub fn client(&self) -> CkbRpcClient {
        let client = CkbRpcClient::new(self.url.as_str());
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