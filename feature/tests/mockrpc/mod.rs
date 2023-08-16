use reqwest::blocking::Client;


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


pub struct MockData {
    pub(crate) url: String,
    pub(crate) request_data: serde_json::Value,
    pub(crate) response_data: serde_json::Value,
}