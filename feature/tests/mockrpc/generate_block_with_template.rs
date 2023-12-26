use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("generate_block_with_template", "[block_template]"))]
fn generate_block_with_template_with_block_template(mock_rpc_data: MockRpcData) {
    let client = mock_rpc_data.client();
    let ret = client.generate_block_with_template(
        serde_json::from_value(
            mock_rpc_data.request_data["params"][0].clone()
        ).unwrap()
    ).unwrap();
    assert_eq!(ret,
               serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
