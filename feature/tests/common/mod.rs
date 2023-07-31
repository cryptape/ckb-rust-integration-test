use ckb_sdk::CkbRpcClient;

pub fn setup(){
    //!todo 公共部分完成环境预处理准备
}

pub const CKB_DEVNET: &str = "https://testnet.ckbapp.dev/";

pub const CKB_DEVNET2: &str = "https://testnet.ckb.dev/";

pub const CKB_MOCKNET: &str = "http://127.0.0.1:5000/test";

pub fn remove_quotes(json_string: &String) -> &str {
    // 去除字符串开头和结尾的双引号
    if json_string.starts_with('"') && json_string.ends_with('"') {
        &json_string[1..json_string.len() - 1]
    } else {
        json_string
    }
}