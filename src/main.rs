use dotenv::dotenv;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let enclave_key = hex::encode(fs::read("./app/secp.sec").unwrap());
    println!("enclave key: {}", enclave_key);

    // "entity_registry": "0xeeB96A7d31B56AD48aDc85908f01B20689F7E17f",
    // "generator_registry": "0x103e9C0e8E0A745A41F8A52142F452E7f8fAaCAd",
    // "proof_market_place": "0x6441dcD0f88f70E912A873baaeC5d02e564Ebc78",
    let matching_engine = kalypso_matching_engine::MatchingEngine::new(
        "http://88.99.141.248:8545".into(),
        "987".into(),
        enclave_key,
        "c94cd379780ee3984f467cd78b2bc51b883c9a76fb44a2d45fb6dd7a169356aa".into(),
        "0x6441dcD0f88f70E912A873baaeC5d02e564Ebc78".into(),
        "0x103e9C0e8E0A745A41F8A52142F452E7f8fAaCAd".into(),
        "0xeeB96A7d31B56AD48aDc85908f01B20689F7E17f".into(),
        "1".into(),
    );

    let _ = matching_engine.run().await;

    Ok(())
}
