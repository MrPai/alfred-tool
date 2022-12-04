use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Summary {
    pub specification: String,
    pub schema: Schema,
    pub registry: Vec<ParaInfo>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct Schema {
    prefix: Option<String>,
    network: Option<String>,
    displayName: Option<String>,
    symbols: Option<String>,
    decimals: Option<String>,
    standardAccount: Option<String>,
    website: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct ParaInfo {
    pub prefix: u64,
    pub network: String,
    pub displayName: String,
    pub symbols: Vec<String>,
    pub decimals: Vec<u64>,
    pub standardAccount: Option<String>,
    pub website: Option<String>,
}
