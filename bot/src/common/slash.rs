use serenity::model::interactions::ApplicationCommandInteractionData;

pub fn extract_option(data: &ApplicationCommandInteractionData, default: String) -> String {
    let opt = data
        .options
        .iter()
        .find(|&opt| opt.name == "lang")
        .map(|v| v.value.clone());
    if let Some(Some(arg)) = opt {
        use serde_json::Value::*;
        match arg {
            String(s) => s,
            Null => "".to_string(),
            Bool(b) => if b { "true".to_string() } else { "false".to_string() }
            Number(n) => { format!("{}", n) }
            v => v.to_string()
        }
    } else {
        default
    }
}