use enum_product::enum_product;

enum_product!(pub enum Code {
    ["ja-JP", "en-US", "en-UK"],
    ["Standard", "Wavenet"],
    ["A", "B", "C", "D"],
});

fn main() {
    let code = Code::JaJPStandardA;
    assert_eq!(code.to_string(), "ja-JPStandardA".to_string())
}
