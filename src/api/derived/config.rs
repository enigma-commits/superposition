// TODO :: Handle errors with appropriate error message

use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Json},
    Either::Left,
    HttpRequest,
};
use serde_json::{to_value, Error, Value};

use crate::api::primary::{
    context_overrides::fetch_override_from_ctx_id, global_config::get_complete_config,
    new_contexts::fetch_new_contexts, overrides::get_override_helper,
};

use crate::utils::errors::{
    AppError,
    AppErrorType::{DBError, SomethingWentWrong},
};
use crate::utils::helpers::strip_double_quotes;

use crate::AppState;

fn default_parsing_error(err: Error) -> AppError {
    AppError {
        message: None,
        cause: Some(Left(err.to_string())),
        status: SomethingWentWrong,
    }
}

async fn get_new_context_overrides_object(
    state: &Data<AppState>,
    query_string: &str,
) -> Result<Value, AppError> {
    let raw_contexts = fetch_new_contexts(state, query_string.to_string()).await?;

    let mut contexts = Vec::new();
    let mut override_map = HashMap::new();

    for item in raw_contexts {
        match (item.get("id"), item.get("condition")) {
            (Some(context_id), Some(condition)) => {
                if let Ok(override_id) =
                    fetch_override_from_ctx_id(&state, strip_double_quotes(&context_id.to_string()))
                        .await
                {
                    let fetched_override_value =
                        get_override_helper(&state, override_id.to_owned()).await?;

                    override_map.insert(override_id.to_owned(), fetched_override_value);

                    contexts.push(
                        to_value(HashMap::from([
                            (
                                "overrideWithKeys",
                                &to_value(override_id).map_err(default_parsing_error)?,
                            ),
                            ("condition", condition),
                        ]))
                        .map_err(|err| AppError {
                            message: None,
                            cause: Some(Left(err.to_string())),
                            status: DBError,
                        })?,
                    );
                }
            }
            (_, _) => (),
        }
    }

    to_value(HashMap::from([
        (
            "context",
            to_value(contexts).map_err(default_parsing_error)?,
        ),
        (
            "overrides",
            to_value(override_map).map_err(default_parsing_error)?,
        ),
    ]))
    .map_err(|err| AppError {
        message: None,
        cause: Some(Left(err.to_string())),
        status: DBError,
    })
}

#[get("")]
pub async fn get_config(state: Data<AppState>, req: HttpRequest) -> Result<Json<Value>, AppError> {
    let query_string = req.query_string();

    Ok(Json(
        to_value(HashMap::from([
            ("global_config", get_complete_config(&state).await?),
            // ("context_overrides", get_context_overrides_object(&state, query_string).await?)
            (
                "context_overrides",
                get_new_context_overrides_object(&state, query_string).await?,
            ),
        ]))
        .map_err(|err| AppError {
            message: None,
            cause: Some(Left(err.to_string())),
            status: DBError,
        })?,
    ))
}
