//! HTTP 请求处理命令
//! 处理来自前端的 HTTP 请求，自动携带 token 进行鉴权

use crate::config::Config;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    #[serde(rename = "__token")]
    pub token: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub data: Option<Value>,
}

/// 处理 HTTP 请求的 Tauri command
#[tauri::command]
pub async fn http_request(
    request: HttpRequest,
    config: State<'_, Config>,
) -> Result<Value, String> {
    // 构建完整 URL
    let base_url = format!("http://{}:{}", config.host, config.port);
    let full_url = if request.url.starts_with('/') {
        format!("{}{}", base_url, request.url)
    } else {
        format!("{}/{}", base_url, request.url)
    };

    // 创建 HTTP 客户端
    let client = reqwest::Client::new();
    let mut req_builder = match request.method.to_uppercase().as_str() {
        "GET" => client.get(&full_url),
        "POST" => client.post(&full_url),
        "PUT" => client.put(&full_url),
        "DELETE" => client.delete(&full_url),
        "PATCH" => client.patch(&full_url),
        _ => return Err(format!("Unsupported HTTP method: {}", request.method)),
    };

    // 添加 Authorization 头
    if let Some(token) = &request.token {
        if !token.is_empty() {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", token));
        }
    }

    // 添加查询参数
    if let Some(params) = &request.params {
        req_builder = req_builder.query(params);
    }

    // 添加请求体
    if let Some(data) = &request.data {
        req_builder = req_builder.json(data);
    }

    // 发送请求
    let response = req_builder
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    // 检查响应状态
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("HTTP {} - {}", status, error_text));
    }

    // 解析响应
    let result = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(result)
}
