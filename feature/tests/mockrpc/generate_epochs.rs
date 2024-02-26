use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("generate_epochs", "[]"))]
fn generate_epochs(mock_rpc_data: MockRpcData) {
    let client = mock_rpc_data.client();
    let epochs = client.generate_epochs().unwrap();
    assert_eq!(epochs, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}