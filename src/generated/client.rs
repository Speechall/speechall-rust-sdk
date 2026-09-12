//! Reqwest client generated from the OpenAPI document (main spec §8 Output A).
//!
//! Bounded JSON/form bodies (§34), streaming raw payloads (§32), exhaustive documented-status enums (§2.4), typed documented response headers (§15), redirects off by default (§30.1), and the authoritative `ClientError` (§36). Recorded decision for multi-content statuses WITH documented headers: the typed fields hoist onto the status VARIANT beside the content enum. The source document declares OpenAPI 3.0.0.
//!
//! Servers (companion §8): operation-level `servers` override path-level, path-level overrides root-level, and within each effective array the first entry is that operation's default base. Every DISTINCT effective default URL becomes its own stored base: `base_url` is the primary (the first operation's first effective server); further bases live in `base_url_<key>` fields whose keys are documented under `ClientBuilder::secondary_base_url`. Recorded decision: an explicit `base_url` replaces ONLY the primary base; each other base needs its own `secondary_base_url` override, so a relative secondary still requires an absolute value there (D-impl-relative-servers).
//! Generated deterministically byte-for-byte (main spec §50 test 39); do not edit by hand.
use super::models::{
    CreateReplacementRulesetRequestBody, CreateReplacementRulesetResponseBody, ErrorResponse,
    RemoteTranscriptionConfiguration, TranscriptLanguageCode, TranscriptOutputFormat,
    TranscriptionResponseFallback,
};
use ::openapi_support::client_error::{BodyLimitDirection, ClientError};
use ::openapi_support::collect::collect_reqwest_limited;
use ::openapi_support::encode::serialize_json_limited;
use ::openapi_support::limits::BodyLimits;
use ::openapi_support::mediatype::{match_entry, ParsedMediaType};
use ::openapi_support::params::{encode_query_pairs, ParamSpec, ParamStyle, ParamValue};

/// Client carrying one resolved base per distinct effective default server (companion §8): `base_url` is the PRIMARY base (the first operation's first effective server); every further distinct URL gets its own `base_url_<key>` field, and each generated method sends through its operation's own base.
#[derive(Clone)]
pub struct Client {
    http: ::reqwest::Client,
    base_url: String,
    limits: BodyLimits,
}

/// Builder for `Client` (main spec §30.1): redirects disabled unless opted in through `follow_redirects`; relative default servers require explicit overrides (D-impl-relative-servers). Recorded decision (companion §8): an explicit `base_url` replaces ONLY the primary base; every additional base is overridden per key through `secondary_base_url`.
pub struct ClientBuilder {
    http: ::reqwest::ClientBuilder,
    base_url: Option<String>,
    limits: BodyLimits,
    default_server_url: String,
    default_server_variables: Vec<(String, String, Option<Vec<String>>)>,
    server_variables: ::std::collections::BTreeMap<String, String>,
    auth_bearer_auth: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    /// Process-default transport: no redirects (§30.1) and process-default body limits (§33).
    #[must_use]
    pub fn new() -> Self {
        let default_server_url = "https://api.speechall.com/v1".to_owned();
        let default_server_variables = Vec::new();
        Self {
            http: ::reqwest::Client::builder().redirect(::reqwest::redirect::Policy::none()),
            base_url: None,
            limits: BodyLimits::process_default(),
            default_server_url,
            default_server_variables,
            server_variables: ::std::collections::BTreeMap::new(),
            auth_bearer_auth: None,
        }
    }

    /// Overrides the resolved PRIMARY base URL only (recorded companion §8 decision); required before `build` when the primary default server is not absolute. Secondary bases are overridden through `secondary_base_url`.
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
        self
    }

    /// Replaces the process-default body limits (main spec §33).
    pub fn limits(mut self, limits: BodyLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Opts into redirect following (§30.1); generated decoding never buffers bodies to enable replay.
    pub fn follow_redirects(mut self, policy: ::reqwest::redirect::Policy) -> Self {
        self.http = self.http.redirect(policy);
        self
    }

    /// Merges extra headers into every request sent through the built client (issue #12 escape hatch): covers auth schemes without a typed method plus unrelated needs like `User-Agent`. Typed credentials, when configured, are applied at `build` time on top of these headers.
    pub fn default_headers(mut self, headers: ::http::HeaderMap) -> Self {
        self.http = self.http.default_headers(headers);
        self
    }

    /// HTTP bearer credentials from security scheme `bearerAuth` (issue #12): sends `Authorization: Bearer <token>` on every request.
    pub fn bearer_auth(mut self, token: impl Into<String>) -> Self {
        self.auth_bearer_auth = Some(token.into());
        self
    }

    /// Builds the client (main spec §30.1, companion §8): every distinct base resolves independently — builder overrides or declared defaults, validated against their enums — and a non-absolute base without its own override is `ClientError::InvalidUrl` (D-impl-relative-servers).
    /// Stored credentials (the typed auth methods) are applied on top of `default_headers` here; a value that cannot become a header is `ClientError::InvalidHeader`.
    pub fn build(self) -> Result<Client, ClientError> {
        let base_url = match self.base_url {
            Some(explicit) => explicit,
            None => substitute_server_variables(
                &self.default_server_url,
                &self.default_server_variables,
                &self.server_variables,
            )?,
        };
        let trimmed = base_url.trim_end_matches('/');
        if !is_absolute_url(trimmed) {
            return Err(ClientError::InvalidUrl(format!(
                "base URL `{trimmed}` is not absolute; call `base_url` because no \
         absolute default server exists"
            )));
        }
        let mut auth_headers = ::http::HeaderMap::new();
        if let Some(token) = self.auth_bearer_auth {
            let value =
                ::http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|source| {
                    ClientError::InvalidHeader {
                        name: ::http::header::AUTHORIZATION,
                        source: Box::new(source),
                    }
                })?;
            auth_headers.insert(::http::header::AUTHORIZATION, value);
        }
        let http = self
            .http
            .default_headers(auth_headers)
            .build()
            .map_err(ClientError::Transport)?;
        Ok(Client {
            http,
            base_url: trimmed.to_owned(),
            limits: self.limits,
        })
    }
}

/// Documented representations for status 200 of `transcribe` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum Transcribe200Content {
    Json(TranscriptionResponseFallback),
    TextPlain(String),
}

/// Typed payload for status 429 of `transcribe` (main spec §15 Output A): required headers as plain fields, optional headers as `Option<T>`, then the decoded body.
#[derive(Debug)]
pub struct Transcribe429 {
    /// Documented response header `Retry-After` (optional).
    pub retry_after: Option<i64>,
    pub body: ErrorResponse,
}

/// Documented representations for status 500 of `transcribe` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum Transcribe500Content {
    Json(ErrorResponse),
    TextPlain(String),
}

/// Documented outcomes for `transcribe` (main spec §8/§13): exhaustive match required; deliberately not `#[non_exhaustive]` (§47).
#[derive(Debug)]
pub enum TranscribeResponse {
    /// HTTP 200 Ok.
    Ok200(Transcribe200Content),
    /// HTTP 400 BadRequest.
    BadRequest400(ErrorResponse),
    /// HTTP 401 Unauthorized.
    Unauthorized401(ErrorResponse),
    /// HTTP 402 PaymentRequired.
    PaymentRequired402(ErrorResponse),
    /// HTTP 404 NotFound.
    NotFound404(ErrorResponse),
    /// HTTP 429 TooManyRequests.
    TooManyRequests429(Transcribe429),
    /// HTTP 500 InternalServerError.
    InternalServerError500(Transcribe500Content),
    /// HTTP 503 ServiceUnavailable.
    ServiceUnavailable503(ErrorResponse),
    /// HTTP 504 GatewayTimeout.
    GatewayTimeout504(ErrorResponse),
}

/// Documented representations for status 200 of `transcribe_remote` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum TranscribeRemote200Content {
    Json(TranscriptionResponseFallback),
    TextPlain(String),
}

/// Typed payload for status 429 of `transcribe_remote` (main spec §15 Output A): required headers as plain fields, optional headers as `Option<T>`, then the decoded body.
#[derive(Debug)]
pub struct TranscribeRemote429 {
    /// Documented response header `Retry-After` (optional).
    pub retry_after: Option<i64>,
    pub body: ErrorResponse,
}

/// Documented representations for status 500 of `transcribe_remote` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum TranscribeRemote500Content {
    Json(ErrorResponse),
    TextPlain(String),
}

/// Documented outcomes for `transcribe_remote` (main spec §8/§13): exhaustive match required; deliberately not `#[non_exhaustive]` (§47).
#[derive(Debug)]
pub enum TranscribeRemoteResponse {
    /// HTTP 200 Ok.
    Ok200(TranscribeRemote200Content),
    /// HTTP 400 BadRequest.
    BadRequest400(ErrorResponse),
    /// HTTP 401 Unauthorized.
    Unauthorized401(ErrorResponse),
    /// HTTP 402 PaymentRequired.
    PaymentRequired402(ErrorResponse),
    /// HTTP 404 NotFound.
    NotFound404(ErrorResponse),
    /// HTTP 429 TooManyRequests.
    TooManyRequests429(TranscribeRemote429),
    /// HTTP 500 InternalServerError.
    InternalServerError500(TranscribeRemote500Content),
    /// HTTP 503 ServiceUnavailable.
    ServiceUnavailable503(ErrorResponse),
    /// HTTP 504 GatewayTimeout.
    GatewayTimeout504(ErrorResponse),
}

/// Typed payload for status 429 of `create_replacement_ruleset` (main spec §15 Output A): required headers as plain fields, optional headers as `Option<T>`, then the decoded body.
#[derive(Debug)]
pub struct CreateReplacementRuleset429 {
    /// Documented response header `Retry-After` (optional).
    pub retry_after: Option<i64>,
    pub body: ErrorResponse,
}

/// Documented representations for status 500 of `create_replacement_ruleset` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum CreateReplacementRuleset500Content {
    Json(ErrorResponse),
    TextPlain(String),
}

/// Documented outcomes for `create_replacement_ruleset` (main spec §8/§13): exhaustive match required; deliberately not `#[non_exhaustive]` (§47).
#[derive(Debug)]
pub enum CreateReplacementRulesetResponse {
    /// HTTP 201 Created.
    Created201(CreateReplacementRulesetResponseBody),
    /// HTTP 400 BadRequest.
    BadRequest400(ErrorResponse),
    /// HTTP 401 Unauthorized.
    Unauthorized401(ErrorResponse),
    /// HTTP 402 PaymentRequired.
    PaymentRequired402(ErrorResponse),
    /// HTTP 429 TooManyRequests.
    TooManyRequests429(CreateReplacementRuleset429),
    /// HTTP 500 InternalServerError.
    InternalServerError500(CreateReplacementRuleset500Content),
    /// HTTP 503 ServiceUnavailable.
    ServiceUnavailable503(ErrorResponse),
    /// HTTP 504 GatewayTimeout.
    GatewayTimeout504(ErrorResponse),
}

/// Typed payload for status 429 of `list_speech_to_text_models` (main spec §15 Output A): required headers as plain fields, optional headers as `Option<T>`, then the decoded body.
#[derive(Debug)]
pub struct ListSpeechToTextModels429 {
    /// Documented response header `Retry-After` (optional).
    pub retry_after: Option<i64>,
    pub body: ErrorResponse,
}

/// Documented representations for status 500 of `list_speech_to_text_models` (main spec §11): the client negotiates via Content-Type (§28).
#[derive(Debug)]
pub enum ListSpeechToTextModels500Content {
    Json(ErrorResponse),
    TextPlain(String),
}

/// Documented outcomes for `list_speech_to_text_models` (main spec §8/§13): exhaustive match required; deliberately not `#[non_exhaustive]` (§47).
#[derive(Debug)]
pub enum ListSpeechToTextModelsResponse {
    /// HTTP 200 Ok.
    Ok200(Vec<serde_json::Value>),
    /// HTTP 400 BadRequest.
    BadRequest400(ErrorResponse),
    /// HTTP 401 Unauthorized.
    Unauthorized401(ErrorResponse),
    /// HTTP 402 PaymentRequired.
    PaymentRequired402(ErrorResponse),
    /// HTTP 404 NotFound.
    NotFound404(ErrorResponse),
    /// HTTP 429 TooManyRequests.
    TooManyRequests429(ListSpeechToTextModels429),
    /// HTTP 500 InternalServerError.
    InternalServerError500(ListSpeechToTextModels500Content),
    /// HTTP 503 ServiceUnavailable.
    ServiceUnavailable503(ErrorResponse),
    /// HTTP 504 GatewayTimeout.
    GatewayTimeout504(ErrorResponse),
}

impl Client {
    /// `POST` `/transcribe`.
    /// Operation `transcribe`.
    pub async fn transcribe(
        &self,
        model: &str,
        language: Option<TranscriptLanguageCode>,
        output_format: Option<TranscriptOutputFormat>,
        ruleset_id: Option<&str>,
        punctuation: Option<bool>,
        diarization: Option<bool>,
        initial_prompt: Option<&str>,
        temperature: Option<f64>,
        speakers_expected: Option<i64>,
        custom_vocabulary: Option<&[String]>,
        body: ::reqwest::Body,
    ) -> Result<TranscribeResponse, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/transcribe");
        let mut query_pairs: Vec<(String, String)> = Vec::new();
        let spec = ParamSpec::new("model", ParamStyle::Form, true, false);
        let value = ParamValue::Text(model.to_owned());
        query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
            ClientError::InvalidUrl(format!("parameter `model` serialization failed: {error}"))
        })?);
        let spec = ParamSpec::new("language", ParamStyle::Form, true, false);
        if let Some(raw) = language {
            let value = ParamValue::from_serde(&raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `language` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("output_format", ParamStyle::Form, true, false);
        if let Some(raw) = output_format {
            let value = ParamValue::from_serde(&raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `output_format` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("ruleset_id", ParamStyle::Form, true, false);
        if let Some(raw) = ruleset_id {
            let value = ParamValue::Text(raw.to_owned());
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `ruleset_id` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("punctuation", ParamStyle::Form, true, false);
        if let Some(raw) = punctuation {
            let value = ParamValue::Bool(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `punctuation` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("diarization", ParamStyle::Form, true, false);
        if let Some(raw) = diarization {
            let value = ParamValue::Bool(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `diarization` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("initial_prompt", ParamStyle::Form, true, false);
        if let Some(raw) = initial_prompt {
            let value = ParamValue::Text(raw.to_owned());
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `initial_prompt` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("temperature", ParamStyle::Form, true, false);
        if let Some(raw) = temperature {
            let value = ParamValue::Float(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `temperature` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("speakers_expected", ParamStyle::Form, true, false);
        if let Some(raw) = speakers_expected {
            let value = ParamValue::Int(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `speakers_expected` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("custom_vocabulary", ParamStyle::Form, true, false);
        if let Some(raw) = custom_vocabulary {
            let value = ParamValue::Array(
                raw.iter()
                    .map(|item| ParamValue::Text(item.clone()))
                    .collect::<Vec<_>>(),
            );
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `custom_vocabulary` serialization failed: {error}"
                ))
            })?);
        }
        if !query_pairs.is_empty() {
            url.push('?');
            for (index, (name, value)) in query_pairs.iter().enumerate() {
                if index > 0 {
                    url.push('&');
                }
                url.push_str(name);
                url.push('=');
                url.push_str(value);
            }
        }
        // §30.1: redirects are off by default so documented 3xx statuses reach the exhaustive enum; opt-in following never buffers bodies for replay.
        let response = self
            .http
            .request(::http::Method::POST, &url)
            .header(::http::header::CONTENT_TYPE, "audio/*")
            .header(::http::header::ACCEPT, "application/json, text/plain")
            .body(body)
            .send()
            .await?;
        self.decode_transcribe(response).await
    }

    /// Shared decode tail for `transcribe` (main spec §23–§28): classifies the received response into its exhaustive documented-status enum.
    /// Called by `transcribe` and its §31 `transcribe_replaying` twin so both share one classification path.
    #[allow(clippy::unused_async)]
    async fn decode_transcribe(
        &self,
        response: ::reqwest::Response,
    ) -> Result<TranscribeResponse, ClientError> {
        match response.status() {
            ::http::StatusCode::OK => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.structured_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: TranscriptionResponseFallback =
                            json_decode(&bytes, Some(content_type))?;
                        let payload = Transcribe200Content::Json(value);
                        Ok(TranscribeResponse::Ok200(payload))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.structured_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = Transcribe200Content::TextPlain(value);
                        Ok(TranscribeResponse::Ok200(payload))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::BAD_REQUEST => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::BadRequest400(value))
            }
            ::http::StatusCode::UNAUTHORIZED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::Unauthorized401(value))
            }
            ::http::StatusCode::PAYMENT_REQUIRED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::PaymentRequired402(value))
            }
            ::http::StatusCode::NOT_FOUND => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::NotFound404(value))
            }
            ::http::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = parse_optional_header::<i64>(&response, "retry-after")?;
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::TooManyRequests429(Transcribe429 {
                    retry_after,
                    body: value,
                }))
            }
            ::http::StatusCode::INTERNAL_SERVER_ERROR => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                        let payload = Transcribe500Content::Json(value);
                        Ok(TranscribeResponse::InternalServerError500(payload))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = Transcribe500Content::TextPlain(value);
                        Ok(TranscribeResponse::InternalServerError500(payload))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::SERVICE_UNAVAILABLE => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::ServiceUnavailable503(value))
            }
            ::http::StatusCode::GATEWAY_TIMEOUT => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeResponse::GatewayTimeout504(value))
            }
            other => Err(ClientError::UndocumentedStatus { status: other }),
        }
    }

    /// `POST` `/transcribe` with explicit-factory retries (§31/D-impl-retry).
    /// Operation `transcribe`.
    /// Idempotency is the caller's responsibility: PUT-style operations are natural fits; retrying POST may duplicate effects.
    /// Every attempt rebuilds the streaming body through `body_factory`; multipart-free raw payloads are never buffered for replay.
    /// Only PRE-response transport failures classified by `openapi_support::retry::is_retryable_transport` are retried — once response headers arrive the outcome is final; factory errors abort without retry.
    pub async fn transcribe_replaying<F, Fut>(
        &self,
        model: &str,
        language: Option<TranscriptLanguageCode>,
        output_format: Option<TranscriptOutputFormat>,
        ruleset_id: Option<&str>,
        punctuation: Option<bool>,
        diarization: Option<bool>,
        initial_prompt: Option<&str>,
        temperature: Option<f64>,
        speakers_expected: Option<i64>,
        custom_vocabulary: Option<&[String]>,
        body_factory: F,
        policy: ::openapi_support::retry::RetryPolicy,
    ) -> Result<TranscribeResponse, ClientError>
    where
        F: Fn() -> Fut,
        Fut: ::std::future::Future<Output = Result<::reqwest::Body, ClientError>>,
    {
        let mut url = self.base_url.clone();
        url.push_str("/transcribe");
        let mut query_pairs: Vec<(String, String)> = Vec::new();
        let spec = ParamSpec::new("model", ParamStyle::Form, true, false);
        let value = ParamValue::Text(model.to_owned());
        query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
            ClientError::InvalidUrl(format!("parameter `model` serialization failed: {error}"))
        })?);
        let spec = ParamSpec::new("language", ParamStyle::Form, true, false);
        if let Some(raw) = language {
            let value = ParamValue::from_serde(&raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `language` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("output_format", ParamStyle::Form, true, false);
        if let Some(raw) = output_format {
            let value = ParamValue::from_serde(&raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `output_format` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("ruleset_id", ParamStyle::Form, true, false);
        if let Some(raw) = ruleset_id {
            let value = ParamValue::Text(raw.to_owned());
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `ruleset_id` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("punctuation", ParamStyle::Form, true, false);
        if let Some(raw) = punctuation {
            let value = ParamValue::Bool(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `punctuation` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("diarization", ParamStyle::Form, true, false);
        if let Some(raw) = diarization {
            let value = ParamValue::Bool(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `diarization` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("initial_prompt", ParamStyle::Form, true, false);
        if let Some(raw) = initial_prompt {
            let value = ParamValue::Text(raw.to_owned());
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `initial_prompt` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("temperature", ParamStyle::Form, true, false);
        if let Some(raw) = temperature {
            let value = ParamValue::Float(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `temperature` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("speakers_expected", ParamStyle::Form, true, false);
        if let Some(raw) = speakers_expected {
            let value = ParamValue::Int(raw);
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `speakers_expected` serialization failed: {error}"
                ))
            })?);
        }
        let spec = ParamSpec::new("custom_vocabulary", ParamStyle::Form, true, false);
        if let Some(raw) = custom_vocabulary {
            let value = ParamValue::Array(
                raw.iter()
                    .map(|item| ParamValue::Text(item.clone()))
                    .collect::<Vec<_>>(),
            );
            query_pairs.extend(encode_query_pairs(&spec, &value).map_err(|error| {
                ClientError::InvalidUrl(format!(
                    "parameter `custom_vocabulary` serialization failed: {error}"
                ))
            })?);
        }
        if !query_pairs.is_empty() {
            url.push('?');
            for (index, (name, value)) in query_pairs.iter().enumerate() {
                if index > 0 {
                    url.push('&');
                }
                url.push_str(name);
                url.push('=');
                url.push_str(value);
            }
        }
        let budget = policy.max_attempts.max(1);
        let mut failed = 0_u32;
        loop {
            let body = (body_factory)().await?;
            let mut request = self.http.request(::http::Method::POST, &url);
            request = request
                .header(::http::header::CONTENT_TYPE, "audio/*")
                .body(body);
            request = request.header(::http::header::ACCEPT, "application/json, text/plain");
            let response = request.send().await;
            match response {
                Ok(response) => return self.decode_transcribe(response).await,
                Err(error) => {
                    failed += 1;
                    let keep_retrying =
                        failed < budget && ::openapi_support::retry::is_retryable_transport(&error);
                    if !keep_retrying {
                        return Err(ClientError::Transport(error));
                    }
                    ::openapi_support::retry::backoff_sleep(policy, failed).await;
                }
            }
        }
    }

    /// `POST` `/transcribe-remote`.
    /// Operation `transcribeRemote`.
    pub async fn transcribe_remote(
        &self,
        body: &RemoteTranscriptionConfiguration,
    ) -> Result<TranscribeRemoteResponse, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/transcribe-remote");
        let payload = match serialize_json_limited(body, self.limits.structured_encode_bytes) {
            Ok(payload) => payload,
            Err(_) => return Err(encode_overflow_error(self.limits.structured_encode_bytes)),
        };
        // §30.1: redirects are off by default so documented 3xx statuses reach the exhaustive enum; opt-in following never buffers bodies for replay.
        let response = self
            .http
            .request(::http::Method::POST, &url)
            .header(::http::header::CONTENT_TYPE, "application/json")
            .header(::http::header::ACCEPT, "application/json, text/plain")
            .body(payload)
            .send()
            .await?;
        self.decode_transcribe_remote(response).await
    }

    /// Shared decode tail for `transcribe_remote` (main spec §23–§28): classifies the received response into its exhaustive documented-status enum.
    #[allow(clippy::unused_async)]
    async fn decode_transcribe_remote(
        &self,
        response: ::reqwest::Response,
    ) -> Result<TranscribeRemoteResponse, ClientError> {
        match response.status() {
            ::http::StatusCode::OK => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.structured_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: TranscriptionResponseFallback =
                            json_decode(&bytes, Some(content_type))?;
                        let payload = TranscribeRemote200Content::Json(value);
                        Ok(TranscribeRemoteResponse::Ok200(payload))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.structured_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = TranscribeRemote200Content::TextPlain(value);
                        Ok(TranscribeRemoteResponse::Ok200(payload))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::BAD_REQUEST => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::BadRequest400(value))
            }
            ::http::StatusCode::UNAUTHORIZED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::Unauthorized401(value))
            }
            ::http::StatusCode::PAYMENT_REQUIRED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::PaymentRequired402(value))
            }
            ::http::StatusCode::NOT_FOUND => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::NotFound404(value))
            }
            ::http::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = parse_optional_header::<i64>(&response, "retry-after")?;
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::TooManyRequests429(
                    TranscribeRemote429 {
                        retry_after,
                        body: value,
                    },
                ))
            }
            ::http::StatusCode::INTERNAL_SERVER_ERROR => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                        let payload = TranscribeRemote500Content::Json(value);
                        Ok(TranscribeRemoteResponse::InternalServerError500(payload))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = TranscribeRemote500Content::TextPlain(value);
                        Ok(TranscribeRemoteResponse::InternalServerError500(payload))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::SERVICE_UNAVAILABLE => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::ServiceUnavailable503(value))
            }
            ::http::StatusCode::GATEWAY_TIMEOUT => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(TranscribeRemoteResponse::GatewayTimeout504(value))
            }
            other => Err(ClientError::UndocumentedStatus { status: other }),
        }
    }

    /// `POST` `/replacement-rulesets`.
    /// Operation `createReplacementRuleset`.
    pub async fn create_replacement_ruleset(
        &self,
        body: &CreateReplacementRulesetRequestBody,
    ) -> Result<CreateReplacementRulesetResponse, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/replacement-rulesets");
        let payload = match serialize_json_limited(body, self.limits.structured_encode_bytes) {
            Ok(payload) => payload,
            Err(_) => return Err(encode_overflow_error(self.limits.structured_encode_bytes)),
        };
        // §30.1: redirects are off by default so documented 3xx statuses reach the exhaustive enum; opt-in following never buffers bodies for replay.
        let response = self
            .http
            .request(::http::Method::POST, &url)
            .header(::http::header::CONTENT_TYPE, "application/json")
            .header(::http::header::ACCEPT, "application/json, text/plain")
            .body(payload)
            .send()
            .await?;
        self.decode_create_replacement_ruleset(response).await
    }

    /// Shared decode tail for `create_replacement_ruleset` (main spec §23–§28): classifies the received response into its exhaustive documented-status enum.
    #[allow(clippy::unused_async)]
    async fn decode_create_replacement_ruleset(
        &self,
        response: ::reqwest::Response,
    ) -> Result<CreateReplacementRulesetResponse, ClientError> {
        match response.status() {
            ::http::StatusCode::CREATED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.structured_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: CreateReplacementRulesetResponseBody =
                    json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::Created201(value))
            }
            ::http::StatusCode::BAD_REQUEST => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::BadRequest400(value))
            }
            ::http::StatusCode::UNAUTHORIZED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::Unauthorized401(value))
            }
            ::http::StatusCode::PAYMENT_REQUIRED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::PaymentRequired402(value))
            }
            ::http::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = parse_optional_header::<i64>(&response, "retry-after")?;
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::TooManyRequests429(
                    CreateReplacementRuleset429 {
                        retry_after,
                        body: value,
                    },
                ))
            }
            ::http::StatusCode::INTERNAL_SERVER_ERROR => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                        let payload = CreateReplacementRuleset500Content::Json(value);
                        Ok(CreateReplacementRulesetResponse::InternalServerError500(
                            payload,
                        ))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = CreateReplacementRuleset500Content::TextPlain(value);
                        Ok(CreateReplacementRulesetResponse::InternalServerError500(
                            payload,
                        ))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::SERVICE_UNAVAILABLE => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::ServiceUnavailable503(
                    value,
                ))
            }
            ::http::StatusCode::GATEWAY_TIMEOUT => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(CreateReplacementRulesetResponse::GatewayTimeout504(value))
            }
            other => Err(ClientError::UndocumentedStatus { status: other }),
        }
    }

    /// `GET` `/speech-to-text-models`.
    /// Operation `listSpeechToTextModels`.
    pub async fn list_speech_to_text_models(
        &self,
    ) -> Result<ListSpeechToTextModelsResponse, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/speech-to-text-models");
        // §30.1: redirects are off by default so documented 3xx statuses reach the exhaustive enum; opt-in following never buffers bodies for replay.
        let response = self
            .http
            .request(::http::Method::GET, &url)
            .header(::http::header::ACCEPT, "application/json, text/plain")
            .send()
            .await?;
        self.decode_list_speech_to_text_models(response).await
    }

    /// Shared decode tail for `list_speech_to_text_models` (main spec §23–§28): classifies the received response into its exhaustive documented-status enum.
    #[allow(clippy::unused_async)]
    async fn decode_list_speech_to_text_models(
        &self,
        response: ::reqwest::Response,
    ) -> Result<ListSpeechToTextModelsResponse, ClientError> {
        match response.status() {
            ::http::StatusCode::OK => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.structured_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: Vec<serde_json::Value> = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::Ok200(value))
            }
            ::http::StatusCode::BAD_REQUEST => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::BadRequest400(value))
            }
            ::http::StatusCode::UNAUTHORIZED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::Unauthorized401(value))
            }
            ::http::StatusCode::PAYMENT_REQUIRED => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::PaymentRequired402(value))
            }
            ::http::StatusCode::NOT_FOUND => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::NotFound404(value))
            }
            ::http::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = parse_optional_header::<i64>(&response, "retry-after")?;
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::TooManyRequests429(
                    ListSpeechToTextModels429 {
                        retry_after,
                        body: value,
                    },
                ))
            }
            ::http::StatusCode::INTERNAL_SERVER_ERROR => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                let mut best_rank: Option<u8> = None;
                let mut best_index: usize = 0;
                if let Some(rank) = match_entry(&parsed, "application/json") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 0;
                    }
                }
                if let Some(rank) = match_entry(&parsed, "text/plain") {
                    let rank = negotiation_rank(rank);
                    if best_rank.is_none_or(|seen| rank < seen) {
                        best_rank = Some(rank);
                        best_index = 1;
                    }
                }
                let selected = best_rank.is_some().then_some(best_index);
                match selected {
                    Some(0) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        if bytes.is_empty() {
                            return Err(ClientError::Decode {
                                content_type: Some(content_type),
                                source: Box::new(EmptyJsonBody),
                            });
                        }
                        let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                        let payload = ListSpeechToTextModels500Content::Json(value);
                        Ok(ListSpeechToTextModelsResponse::InternalServerError500(
                            payload,
                        ))
                    }
                    Some(1) => {
                        ensure_utf8_charset(&parsed)?;
                        let limit = self.limits.error_response_bytes;
                        let bytes = collect_reqwest_limited(response, limit).await?;
                        let value = text_decode(bytes, Some(content_type))?;
                        let payload = ListSpeechToTextModels500Content::TextPlain(value);
                        Ok(ListSpeechToTextModelsResponse::InternalServerError500(
                            payload,
                        ))
                    }
                    _ => Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned(), "text/plain".to_owned()],
                        actual: Some(mime_of(&parsed)?),
                    }),
                }
            }
            ::http::StatusCode::SERVICE_UNAVAILABLE => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::ServiceUnavailable503(value))
            }
            ::http::StatusCode::GATEWAY_TIMEOUT => {
                let parsed = parse_response_content_type(&response)?;
                let Some(parsed) = parsed else {
                    return Err(ClientError::UnexpectedContentType {
                        expected: vec!["application/json".to_owned()],
                        actual: None,
                    });
                };
                let content_type = mime_of(&parsed)?;
                ensure_utf8_charset(&parsed)?;
                let limit = self.limits.error_response_bytes;
                let bytes = collect_reqwest_limited(response, limit).await?;
                if bytes.is_empty() {
                    return Err(ClientError::Decode {
                        content_type: Some(content_type),
                        source: Box::new(EmptyJsonBody),
                    });
                }
                let value: ErrorResponse = json_decode(&bytes, Some(content_type))?;
                Ok(ListSpeechToTextModelsResponse::GatewayTimeout504(value))
            }
            other => Err(ClientError::UndocumentedStatus { status: other }),
        }
    }
}

/// Reads and parses the response `Content-Type` (§28 steps 1–2): duplicate headers are ambiguous decode errors (§28.1), a missing header yields `None`, malformed values surface as `MalformedContentType`.
fn parse_response_content_type(
    response: &::reqwest::Response,
) -> Result<Option<ParsedMediaType>, ClientError> {
    let values: Vec<&::http::HeaderValue> = response
        .headers()
        .get_all(::http::header::CONTENT_TYPE)
        .iter()
        .collect();
    if values.len() > 1 {
        return Err(ClientError::Decode {
            content_type: None,
            source: Box::new(DuplicateContentType),
        });
    }
    let Some(raw) = values.first() else {
        return Ok(None);
    };
    let text = raw.to_str().map_err(|_| {
        ClientError::MalformedContentType(::openapi_support::mediatype::MalformedContentType)
    })?;
    Ok(Some(::openapi_support::mediatype::parse_content_type(
        text,
    )?))
}

/// duplicate Content-Type headers are an ambiguous message (§28.1); generated code never picks one arbitrarily
#[derive(Debug)]
struct DuplicateContentType;

impl std::fmt::Display for DuplicateContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("duplicate Content-Type headers on one message")
    }
}

impl std::error::Error for DuplicateContentType {}

/// Builds the `mime::Mime` carried by [`ClientError`] fields.
#[allow(clippy::missing_errors_doc)]
fn mime_of(parsed: &ParsedMediaType) -> Result<::mime::Mime, ClientError> {
    let subtype = match &parsed.suffix {
        Some(suffix) => format!("{}+{}", parsed.subtype, suffix),
        None => parsed.subtype.clone(),
    };
    let text = format!("{}/{}", parsed.ty, subtype);
    text.parse().map_err(|_| {
        ClientError::MalformedContentType(::openapi_support::mediatype::MalformedContentType)
    })
}

/// §28.4 charset policy (D-impl-charset-rejection): textual media decode as UTF-8; any other declared charset is a decode error instead of replacement-character corruption.
#[allow(clippy::missing_errors_doc)]
fn ensure_utf8_charset(parsed: &ParsedMediaType) -> Result<(), ClientError> {
    if let Some((_, value)) = parsed.parameters.iter().find(|(name, _)| name == "charset") {
        let lowered = value.to_ascii_lowercase();
        if lowered != "utf-8" && lowered != "utf8" {
            return Err(ClientError::Decode {
                content_type: None,
                source: Box::new(UnsupportedCharset(value.clone())),
            });
        }
    }
    Ok(())
}

/// declared charset is outside the UTF-8 family (§28.4); generated clients surface this as `ClientError::Decode` (D-impl-charset-rejection)
#[derive(Debug)]
struct UnsupportedCharset(String);

impl std::fmt::Display for UnsupportedCharset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "charset `{}` is outside the UTF-8 family", self.0)
    }
}

impl std::error::Error for UnsupportedCharset {}

/// a documented JSON status arrived with an empty body; empty input is never decoded as a default value (§28.3)
#[derive(Debug)]
struct EmptyJsonBody;

impl std::fmt::Display for EmptyJsonBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("documented JSON status arrived with an empty body")
    }
}

impl std::error::Error for EmptyJsonBody {}

/// §28 dispatch ranking: Exact beats suffix family beats range match beats wildcard.
#[must_use]
fn negotiation_rank(matched: ::openapi_support::mediatype::EntryMatch) -> u8 {
    match matched {
        ::openapi_support::mediatype::EntryMatch::Exact => 0,
        ::openapi_support::mediatype::EntryMatch::SuffixFamily => 1,
        ::openapi_support::mediatype::EntryMatch::RangeMatch => 2,
        ::openapi_support::mediatype::EntryMatch::Wildcard => 3,
    }
}

/// Maps bounded JSON decode failures onto [`ClientError::Decode`] (§36); `content_type` is carried for diagnostics.
fn json_decode<T>(bytes: &[u8], content_type: Option<::mime::Mime>) -> Result<T, ClientError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_slice(bytes).map_err(|error| ClientError::Decode {
        content_type,
        source: Box::new(error),
    })
}

/// UTF-8 validation for bounded plain-text bodies (§28.4): invalid bytes are decode errors, never replacement characters.
#[allow(clippy::missing_errors_doc)]
fn text_decode(
    bytes: ::bytes::Bytes,
    content_type: Option<::mime::Mime>,
) -> Result<String, ClientError> {
    ::std::str::from_utf8(&bytes)
        .map(|text| text.to_owned())
        .map_err(|error| ClientError::Decode {
            content_type,
            source: Box::new(error),
        })
}

/// Client-side encode overflow (§34.2): returned BEFORE anything is sent.
#[must_use]
fn encode_overflow_error(limit: usize) -> ClientError {
    ClientError::BodyTooLarge {
        direction: BodyLimitDirection::Encode,
        limit,
    }
}

/// Typed documented response headers (main spec §15): required headers missing from the response are protocol errors (`MissingRequiredHeader`), values failing their Rust type are `InvalidHeader`; both surface BEFORE the body is consumed. A repeated documented header reads its first occurrence.
#[allow(clippy::missing_errors_doc)]
fn parse_optional_header<T>(
    response: &::reqwest::Response,
    wire: &'static str,
) -> Result<Option<T>, ClientError>
where
    T: ::std::str::FromStr,
    T::Err: ::std::error::Error + Send + Sync + 'static,
{
    let name = ::http::HeaderName::from_static(wire);
    match response.headers().get(&name) {
        Some(raw) => parse_header_value(name, raw).map(Some),
        None => Ok(None),
    }
}

/// Decodes one raw header value into its typed representation.
#[allow(clippy::missing_errors_doc)]
fn parse_header_value<T>(
    name: ::http::HeaderName,
    raw: &::http::HeaderValue,
) -> Result<T, ClientError>
where
    T: ::std::str::FromStr,
    T::Err: ::std::error::Error + Send + Sync + 'static,
{
    let text = raw.to_str().map_err(|_| ClientError::InvalidHeader {
        name: name.clone(),
        source: Box::new(NonUtf8HeaderValue),
    })?;
    text.parse().map_err(|source| ClientError::InvalidHeader {
        name,
        source: Box::new(source),
    })
}

/// a documented response header carried non-UTF-8 bytes; generated clients surface this as `ClientError::InvalidHeader`
#[derive(Debug)]
struct NonUtf8HeaderValue;

impl std::fmt::Display for NonUtf8HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("documented response header value is not valid UTF-8")
    }
}

impl std::error::Error for NonUtf8HeaderValue {}

/// Substitutes server variables with builder overrides or declared defaults, validating enum membership at build time (companion §8).
#[allow(clippy::missing_errors_doc)]
fn substitute_server_variables(
    url: &str,
    variables: &[(String, String, Option<Vec<String>>)],
    overrides: &::std::collections::BTreeMap<String, String>,
) -> Result<String, ClientError> {
    let mut resolved = url.to_owned();
    for (name, default, allowed) in variables {
        let value = if let Some(value) = overrides.get(name) {
            value.clone()
        } else {
            default.clone()
        };
        if let Some(allowed) = allowed {
            if !allowed.contains(&value) {
                return Err(ClientError::InvalidUrl(format!(
                    "server variable `{name}` value `{value}` is not one of {allowed:?}"
                )));
            }
        }
        let placeholder = format!("{{{name}}}");
        if !resolved.contains(&placeholder) {
            return Err(ClientError::InvalidUrl(format!(
                "server variable `{name}` has no placeholder in `{url}`"
            )));
        }
        resolved = resolved.replace(&placeholder, &value);
    }
    if resolved.contains('{') || resolved.contains('}') {
        return Err(ClientError::InvalidUrl(format!(
            "unresolved server variable placeholder in `{resolved}`"
        )));
    }
    Ok(resolved)
}

/// Absolute-URL gate for the resolved base (D-impl-relative-servers): scheme + `://` + non-empty remainder.
#[must_use]
fn is_absolute_url(url: &str) -> bool {
    let Some((scheme, rest)) = url.split_once("://") else {
        return false;
    };
    !scheme.is_empty()
        && scheme.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
        && !rest.is_empty()
}
