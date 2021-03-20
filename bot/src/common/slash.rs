use serenity::model::interactions::ApplicationCommandInteractionData;

pub fn extract_option(data: &ApplicationCommandInteractionData, key: &str) -> Option<String> {
    let opt = data
        .options
        .iter()
        .find(|&opt| opt.name == key)
        .map(|v| v.value.clone());
    if let Some(Some(arg)) = opt {
        use serde_json::Value::*;
        let v = match arg {
            String(s) => s,
            Null => "".to_string(),
            Bool(b) => {
                if b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Number(n) => {
                format!("{}", n)
            }
            v => v.to_string(),
        };
        Some(v)
    } else {
        None
    }
}
