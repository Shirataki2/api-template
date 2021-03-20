use lazy_static::lazy_static;
use serde_json::{json, Value};

lazy_static! {
    pub static ref SLASH_COMMANDS: Vec<Value> = vec![json!({
        "name": "lang",
        "description": "Change Bot Language",
        "options": [
            {
                "name": "lang",
                "description": "Language",
                "type": 3,
                "required": true,
                "choices": [
                    {
                        "name": "Japanese",
                        "value": "ja-JP",
                    },
                    {
                        "name": "English",
                        "value": "en-US",
                    },
                ]
            }
        ],
    })];
}
