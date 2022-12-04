use crate::types::ss58_registry::*;
use alfred_primitives::types::*;
use std::fs;
const SS58_REGISTRY: &str = "ss58-registry.json";
pub fn parse_for_subscan_url(json_path: &str, output_dir: &str) {
    let file = fs::File::open(json_path).unwrap();
    let summary: Summary = serde_json::from_reader(file).unwrap();
    let mut items = Items::default();
    for mut para_info in summary.registry.into_iter() {
        let title = para_info.symbols.pop().unwrap_or(para_info.network.clone());
        if para_info.network.eq("heiko") {
            para_info.network = "parallel-heiko".to_string();
        }
        let subtitle = para_info.network.clone();
        let arg = para_info.network;
        let autocomplete = title.clone();
        let item = Item::new(title, subtitle, arg, autocomplete);
        items.items.push(item);
    }
    let json_content = serde_json::to_string(&items);
    if let Ok(json_content) = json_content {
        if let Err(e) = fs::write(
            format!("{}{}", output_dir.to_owned() + "/", SS58_REGISTRY),
            json_content,
        ) {
            log::error!("failed to save data to file: {:?}", e);
        } else {
            log::info!("save price data to file success!");
        }
    } else {
        log::error!("failed to serialize data: {:?}", json_content);
    }
}
