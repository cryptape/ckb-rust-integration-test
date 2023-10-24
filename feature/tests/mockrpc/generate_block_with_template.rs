use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};



#[rstest(mock_rpc_data("generate_block_with_template", "[block_template]"))]
fn generate_block_with_template_with_block_template(mock_rpc_data: MockRpcData) {
    // 在这个示例中，测试用例被跳过
    // 如果有需要，您可以在这里执行一些其他操作
    let _client = mock_rpc_data.client();
    assert!(false,"not support: generate_block_with_template/[block_template]")

}
