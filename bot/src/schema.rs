use lazy_static::lazy_static;
use serde_json::{json, Value};

lazy_static! {
    pub static ref SLASH_COMMANDS: Vec<Value> = vec![
        json!({
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
        }),
        json!({
            "name": "voice",
            "description": "Change Text-to-Speech Voice Model",
            "options": [
                {
                    "name": "model",
                    "description": "Voice Model",
                    "type": 3,
                    "required": true,
                    "choices": [
                        {
                            "name": "JP-Female-Normal-A",
                            "value": "JP-Female-Normal-A"
                        },
                        {
                            "name": "JP-Female-Normal-B",
                            "value": "JP-Female-Normal-B"
                        },
                        {
                            "name": "JP-Female-Premium-A",
                            "value": "JP-Female-Premium-A"
                        },
                        {
                            "name": "JP-Female-Premium-B",
                            "value": "JP-Female-Premium-B"
                        },
                        {
                            "name": "JP-Male-Normal-A",
                            "value": "JP-Male-Normal-A"
                        },
                        {
                            "name": "JP-Male-Normal-B",
                            "value": "JP-Male-Normal-B"
                        },
                        {
                            "name": "JP-Male-Premium-A",
                            "value": "JP-Male-Premium-A"
                        },
                        {
                            "name": "JP-Male-Premium-B",
                            "value": "JP-Male-Premium-B"
                        }
                    ]
                }
            ]
        })
    ];
}
