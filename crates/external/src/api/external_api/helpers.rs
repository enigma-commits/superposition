use actix_web::web::Data;
use experimentation_platform::{
    api::experiments::types::{Variant, VariantType},
    db::models::Experiment,
};

use serde_json::{Map, Value};
use service_utils::{result, service::types::AppState, unexpected_error};

pub fn fetch_variant_id(
    experiment: &Experiment,
    variant: VariantType,
) -> result::Result<String> {
    let variants = &experiment.variants;
    let experiment_variants: Vec<Variant> = serde_json::from_value(variants.clone())
        .map_err(|e| {
            log::error!("parsing to variant type failed with err: {e}");
            unexpected_error!("Something went wrong.")
        })?;

    for ele in experiment_variants {
        if ele.variant_type == variant {
            return Ok(ele.id);
        }
    }
    log::info!(
        "Failed to fetch variant {:?} id for exp {}",
        variant,
        experiment.id
    );
    return Err(unexpected_error!("Something went wrong."));
}

pub async fn get_resolved_config(
    state: &Data<AppState>,
    dimension_ctx: &Map<String, Value>,
) -> result::Result<Value> {
    let http_client = reqwest::Client::new();
    let url = format!("{}/config/resolve", state.cac_host);
    let resp = http_client
        .get(&url)
        .bearer_auth(&state.admin_token)
        .header("x-tenant", "mjos")
        .query(dimension_ctx)
        .send()
        .await
        .map_err(|e| unexpected_error!(e))?
        .json()
        .await
        .map_err(|e| unexpected_error!(e))?;
    Ok(resp)
}
