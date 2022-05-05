use crate::read_state;
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk_macros::query;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[query]
#[trace]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path = request
        .url
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_lowercase();

    match path.as_str() {
        "metrics" => {
            let metrics = read_state(|state| state.metrics());

            let body = serde_json::to_string(&metrics).unwrap().into_bytes();

            HttpResponse {
                status_code: 200,
                headers: vec![
                    HeaderField("Content-Type".to_string(), "application/json".to_string()),
                    HeaderField("Content-Length".to_string(), body.len().to_string()),
                ],
                body: ByteBuf::from(body),
            }
        }
        _ => HttpResponse::not_found(),
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

impl HttpResponse {
    pub fn not_found() -> HttpResponse {
        HttpResponse::status_code(404)
    }

    pub fn status_code(code: u16) -> HttpResponse {
        HttpResponse {
            status_code: code,
            headers: Vec::new(),
            body: ByteBuf::default(),
        }
    }
}
