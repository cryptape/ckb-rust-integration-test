use ckb_sdk::rpc::ckb_light_client::SetScriptsCommand;
use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("set_scripts", "[Vec<ScriptStatus>]"))]
fn set_scripts(mock_rpc_data: MockRpcData) {
    // skip
    // let ckb_client = mock_rpc_data.client();
    //
    // // 反序列化输入参数
    // let scripts = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    //
    // // 调用被测试的函数
    // let tx_hash = ckb_client.set_scripts(
    //     scripts
    // ).unwrap();
    //
    // // 比较输出结果
    // assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}


#[rstest(mock_rpc_data("set_scripts", "[Vec<ScriptStatus>,SetScriptCommand:partial]"))]
fn set_scripts_partial(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    // 反序列化输入参数
    let scripts = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx_hash = ckb_client.set_scripts(
        scripts, Some(SetScriptsCommand::Partial),
    ).unwrap();

    // 比较输出结果
    assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("set_scripts", "[Vec<ScriptStatus>,SetScriptCommand:all]"))]
fn set_scripts_all(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let scripts = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx_hash = ckb_client.set_scripts(
        scripts, Some(SetScriptsCommand::All),
    ).unwrap();

    // 比较输出结果
    assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}

#[rstest(mock_rpc_data("set_scripts", "[Vec<ScriptStatus>,SetScriptCommand:delete]"))]
fn set_scripts_delete(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let scripts = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx_hash = ckb_client.set_scripts(
        scripts, Some(SetScriptsCommand::Delete),
    ).unwrap();

    // 比较输出结果
    assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}


