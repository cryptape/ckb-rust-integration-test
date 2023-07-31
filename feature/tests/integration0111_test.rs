mod mockrpc;

// 使用 tokio-test 宏来支持异步测试
#[tokio::test]
async fn test_estimate_cycles() {
    // 获取方法名和参数
    let method = "estimate_cycles";
    let params = "[tx]";

    // 调用 mock_rpc 函数获取结果
    let result = mockrpc::get_mock_test_data(method, params).await.unwrap();

    // 处理结果，根据方法名和参数进行断言
    match method {
        "estimate_cycles" => {
            println!(
                "Formatted Response Data:\n{}",
                serde_json::to_string_pretty(&result.response_data).unwrap()

            );
        }
        // 添加其他方法的匹配和断言
        _ => {
            // 处理未知方法名的情况
            panic!("Unknown method: {}", method.to_string());
        }
    }
}
