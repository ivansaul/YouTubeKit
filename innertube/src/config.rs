use common::error::Result;
use serde_json::Value;
use std::sync::OnceLock;

pub static DEFAULT_CLIENTS_CACHE: OnceLock<Result<Value>> = OnceLock::new();
pub const DEFAULT_CLIENTS_JSON: &str = r#"
{
    "WEB": {
        "innertube_context": {
            "context": {
                "client": {
                    "clientName": "WEB",
                    "osName": "Windows",
                    "osVersion": "10.0",
                    "clientVersion": "2.20250523.01.00",
                    "platform": "DESKTOP"
                }
            }
        },
        "header": {
            "User-Agent": "Mozilla/5.0",
            "X-Youtube-Client-Name": "1",
            "X-Youtube-Client-Version": "2.20250523.01.00"
        },
        "api_key": "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
        "require_js_player": true,
        "require_po_token": true
    },
    "IOS": {
        "innertube_context": {
            "context": {
                "client": {
                    "clientName": "IOS",
                    "clientVersion": "19.45.4",
                    "deviceMake": "Apple",
                    "platform": "MOBILE",
                    "osName": "iPhone",
                    "osVersion": "18.1.0.22B83",
                    "deviceModel": "iPhone16,2"
                }
            }
        },
        "header": {
            "User-Agent": "com.google.ios.youtube/19.45.4 (iPhone16,2; U; CPU iOS 18_1_0 like Mac OS X;)",
            "X-Youtube-Client-Name": "5"
        },
        "api_key": "AIzaSyB-63vPrdThhKuerbB2N_l7Kwwcxj6yUAc",
        "require_js_player": false,
        "require_po_token": false
    }
}
"#;
