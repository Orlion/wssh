use std::collections::HashMap;

use crate::error;
use rookie::enums::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppItem {
    pub app_id: i64,
    pub app_key: String,
    pub app_name: String,
}

#[derive(Serialize, Deserialize)]
struct AppListResponseData {
    app_list: Vec<AppItem>,
    count: i64,
}

#[derive(Serialize, Deserialize)]
struct AppListResponse {
    error_msg: String,
    error_code: i64,
    data: AppListResponseData,
}

#[derive(Serialize, Deserialize)]
struct PodListResponseData {
    pods: Vec<Pod>,
}

#[derive(Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub ssh_token: String,
    pub pod_ip: String,
}

#[derive(Serialize, Deserialize)]
struct PodListResponse {
    error_msg: String,
    error_code: i64,
    data: PodListResponseData,
}

pub struct Jumpserver {
    cookies: String,
}

impl Jumpserver {
    pub fn new() -> Self {
        Jumpserver { cookies: "".into() }
    }

    pub fn login(&mut self) -> Result<(), error::WsshError> {
        // 使用rookie从chrome获取jumpserver的cookie
        let domains = vec!["jumpserver.domain.com".into()];
        let cookies = rookie::chrome(Some(domains)).map_err(|e| {
            error::from_string(format!("获取jumpserver cookie失败: {}", e.to_string()))
        })?;

        let mut cookie_map: HashMap<String, Cookie> = HashMap::new();
        for cookie in cookies {
            if cookie.name == "sessionid" || cookie.name == "JUMPSERVER_SESS_ID" {
                cookie_map.insert(cookie.name.clone(), cookie);
            }
        }

        self.cookies = cookie_map
            .values()
            .map(|cookie| format!("{}={}", cookie.name, cookie.value))
            .collect::<Vec<String>>()
            .join("; ");

        Ok(())
    }

    pub fn query_app_list(&self, env: &str) -> Result<Vec<AppItem>, error::WsshError> {
        let body = ureq::get("https://jumpserver.domain.com/v1/apps/")
            .query_pairs(vec![("env", env)])
            .set("Cookie", &self.cookies)
            .call()
            .map_err(|e| error::from_string(format!("请求jumpserver应用列表接口失败: {}", e)))?
            .into_string()
            .map_err(|e| {
                error::from_string(format!("jumpserver应用列表接口body获取失败: {}", e))
            })?;
        let app_list_response: AppListResponse = serde_json::from_str(&body).map_err(|e| {
            error::from_string(format!("jumpserver应用列表接口body反序列化失败: {}", e))
        })?;
        if app_list_response.error_code == 0 {
            Ok(app_list_response.data.app_list)
        } else {
            Err(error::from_str(&app_list_response.error_msg))
        }
    }

    pub fn query_pod_list(&self, app_id: i64, env: &str) -> Result<Vec<Pod>, error::WsshError> {
        let body =
            ureq::get(format!("https://jumpserver.domain.com/v1/apps/{}/pods/", app_id).as_str())
                .query_pairs(vec![("app_id", app_id.to_string().as_str()), ("env", env)])
                .set("Cookie", &self.cookies)
                .call()
                .map_err(|e| {
                    error::from_string(format!("请求jumpserver应用Pod列表接口失败: {}", e))
                })?
                .into_string()
                .map_err(|e| {
                    error::from_string(format!("jumpserver应用Pod列表接口body获取失败: {}", e))
                })?;
        let deployment_detail_response: PodListResponse =
            serde_json::from_str(&body).map_err(|e| {
                error::from_string(format!("jumpserver应用Pod列表接口body反序列化失败: {}", e))
            })?;

        if deployment_detail_response.error_code == 0 {
            Ok(deployment_detail_response.data.pods)
        } else {
            Err(error::from_str(&deployment_detail_response.error_msg))
        }
    }
}
