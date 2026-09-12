//! Shared schema models generated from the OpenAPI document (main spec §2.6): one
//! module reused by both client and server operation codecs.
//!
//! Every named `components/schemas` entry appears below in document declaration
//! order; nested anonymous objects and enumerations become generated definitions
//! emitted before their parents (`<Parent><FieldPascal>` plus numeric collision
//! suffixes, companion §10).
//!
//! Property presence/nullability follows companion §2.1 cell-for-cell; bucket-2
//! validation constraints ride as documentation and as emitted `validate_request`
//! methods (companion §9; D-impl-runtime-validation-timing Phase 2 half). This file
//! is generated deterministically byte-for-byte (main spec §50 test 39); do not edit
//! by hand.
use std::collections::BTreeMap;

use openapi_support::optional::OptionalField;
use serde::{Deserialize, Serialize};

/// The identifier for the underlying Speech-to-Text service provider (e.g., 'openai', 'deepgram').
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranscriptionProvider {
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "assemblyai")]
    Assemblyai,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "cloudflare")]
    Cloudflare,
    #[serde(rename = "deepgram")]
    Deepgram,
    #[serde(rename = "elevenlabs")]
    Elevenlabs,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "gladia")]
    Gladia,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "groq")]
    Groq,
    #[serde(rename = "ibm")]
    Ibm,
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "openai")]
    Openai,
    #[serde(rename = "revai")]
    Revai,
    #[serde(rename = "speechmatics")]
    Speechmatics,
    #[serde(rename = "togetherai")]
    Togetherai,
    #[serde(rename = "xai")]
    Xai,
}

/// An opaque Speech-to-Text model identifier in `provider.model_name` form. Obtain currently available values from the `/speech-to-text-models` endpoint.
pub type OpenTranscriptionModelIdentifier = String;

/// Unique identifier for a specific Speech-to-Text model, composed as `provider.model_name`. Used to select the engine for transcription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranscriptionModelIdentifier {
    #[serde(rename = "amazon.transcribe")]
    AmazonTranscribe,
    #[serde(rename = "assemblyai.universal-2")]
    AssemblyaiUniversal2,
    #[serde(rename = "assemblyai.universal-3-5-pro")]
    AssemblyaiUniversal35Pro,
    #[serde(rename = "azure.standard")]
    AzureStandard,
    #[serde(rename = "cloudflare.whisper")]
    CloudflareWhisper,
    #[serde(rename = "cloudflare.whisper-large-v3-turbo")]
    CloudflareWhisperLargeV3Turbo,
    #[serde(rename = "cloudflare.whisper-tiny-en")]
    CloudflareWhisperTinyEn,
    #[serde(rename = "deepgram.base")]
    DeepgramBase,
    #[serde(rename = "deepgram.conversationalai")]
    DeepgramConversationalai,
    #[serde(rename = "deepgram.enhanced")]
    DeepgramEnhanced,
    #[serde(rename = "deepgram.enhanced-finance")]
    DeepgramEnhancedFinance,
    #[serde(rename = "deepgram.enhanced-general")]
    DeepgramEnhancedGeneral,
    #[serde(rename = "deepgram.enhanced-meeting")]
    DeepgramEnhancedMeeting,
    #[serde(rename = "deepgram.enhanced-phonecall")]
    DeepgramEnhancedPhonecall,
    #[serde(rename = "deepgram.finance")]
    DeepgramFinance,
    #[serde(rename = "deepgram.meeting")]
    DeepgramMeeting,
    #[serde(rename = "deepgram.nova")]
    DeepgramNova,
    #[serde(rename = "deepgram.nova-2")]
    DeepgramNova2,
    #[serde(rename = "deepgram.nova-2-automotive")]
    DeepgramNova2Automotive,
    #[serde(rename = "deepgram.nova-2-conversationalai")]
    DeepgramNova2Conversationalai,
    #[serde(rename = "deepgram.nova-2-drivethru")]
    DeepgramNova2Drivethru,
    #[serde(rename = "deepgram.nova-2-finance")]
    DeepgramNova2Finance,
    #[serde(rename = "deepgram.nova-2-general")]
    DeepgramNova2General,
    #[serde(rename = "deepgram.nova-2-medical")]
    DeepgramNova2Medical,
    #[serde(rename = "deepgram.nova-2-meeting")]
    DeepgramNova2Meeting,
    #[serde(rename = "deepgram.nova-2-video")]
    DeepgramNova2Video,
    #[serde(rename = "deepgram.nova-2-voicemail")]
    DeepgramNova2Voicemail,
    #[serde(rename = "deepgram.nova-3")]
    DeepgramNova3,
    #[serde(rename = "deepgram.nova-3-general")]
    DeepgramNova3General,
    #[serde(rename = "deepgram.nova-3-medical")]
    DeepgramNova3Medical,
    #[serde(rename = "deepgram.nova-general")]
    DeepgramNovaGeneral,
    #[serde(rename = "deepgram.nova-medical")]
    DeepgramNovaMedical,
    #[serde(rename = "deepgram.nova-phonecall")]
    DeepgramNovaPhonecall,
    #[serde(rename = "deepgram.phonecall")]
    DeepgramPhonecall,
    #[serde(rename = "deepgram.video")]
    DeepgramVideo,
    #[serde(rename = "deepgram.voicemail")]
    DeepgramVoicemail,
    #[serde(rename = "elevenlabs.scribe-v1")]
    ElevenlabsScribeV1,
    #[serde(rename = "elevenlabs.scribe-v2")]
    ElevenlabsScribeV2,
    #[serde(rename = "gemini.gemini-2.5-flash")]
    GeminiGemini25Flash,
    #[serde(rename = "gemini.gemini-2.5-flash-lite")]
    GeminiGemini25FlashLite,
    #[serde(rename = "gemini.gemini-2.5-pro")]
    GeminiGemini25Pro,
    #[serde(rename = "gladia.standard")]
    GladiaStandard,
    #[serde(rename = "google.enhanced")]
    GoogleEnhanced,
    #[serde(rename = "google.standard")]
    GoogleStandard,
    #[serde(rename = "groq.whisper-large-v3")]
    GroqWhisperLargeV3,
    #[serde(rename = "groq.whisper-large-v3-turbo")]
    GroqWhisperLargeV3Turbo,
    #[serde(rename = "ibm.standard")]
    IbmStandard,
    #[serde(rename = "mistral.voxtral-mini")]
    MistralVoxtralMini,
    #[serde(rename = "mistral.voxtral-mini-v2")]
    MistralVoxtralMiniV2,
    #[serde(rename = "openai.gpt-4o-mini-transcribe")]
    OpenaiGpt4OMiniTranscribe,
    #[serde(rename = "openai.gpt-4o-transcribe")]
    OpenaiGpt4OTranscribe,
    #[serde(rename = "openai.gpt-4o-transcribe-diarize")]
    OpenaiGpt4OTranscribeDiarize,
    #[serde(rename = "openai.whisper-1")]
    OpenaiWhisper1,
    #[serde(rename = "revai.fusion")]
    RevaiFusion,
    #[serde(rename = "revai.machine")]
    RevaiMachine,
    #[serde(rename = "speechmatics.enhanced")]
    SpeechmaticsEnhanced,
    #[serde(rename = "speechmatics.standard")]
    SpeechmaticsStandard,
    #[serde(rename = "togetherai.nvidia-parakeet-tdt-0.6b-v3")]
    TogetheraiNvidiaParakeetTdt06BV3,
    #[serde(rename = "togetherai.thinkingmachines-inkling")]
    TogetheraiThinkingmachinesInkling,
    #[serde(rename = "togetherai.thinkingmachines-inkling-small")]
    TogetheraiThinkingmachinesInklingSmall,
    #[serde(rename = "xai.grok-stt")]
    XaiGrokStt,
}

/// Common configuration options for transcription, applicable to both direct uploads and remote URLs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseTranscriptionConfiguration {
    /// An opaque Speech-to-Text model identifier in `provider.model_name` form. Obtain currently available values from the `/speech-to-text-models` endpoint.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub model: OpenTranscriptionModelIdentifier,
    /// The language code of the audio file, typically in ISO 639-1 format.
    /// Specifying the correct language improves transcription accuracy and speed.
    /// The special value `auto` can be used to request automatic language detection, if supported by the selected model.
    /// If omitted, the default language is English (`en`).
    ///
    /// Default: `"en"`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub language: OptionalField<TranscriptLanguageCode>,
    /// Specifies the desired format of the transcription output.
    /// - `text`: Plain text containing the full transcription.
    /// - `json_text`: A simple JSON object containing the transcription ID and the full text (`TranscriptionOnlyText` schema).
    /// - `json`: A detailed JSON object including segments, timestamps (based on `timestamp_granularity`), language, and potentially speaker labels and provider metadata (`TranscriptionDetailed` schema).
    /// - `srt`: SubRip subtitle format (returned as plain text).
    /// - `vtt`: WebVTT subtitle format (returned as plain text).
    ///
    /// Default: `"text"`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub output_format: OptionalField<TranscriptOutputFormat>,
    /// The unique identifier (UUID) of a pre-defined replacement ruleset to apply to the final transcription text.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `uuid`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub ruleset_id: OptionalField<String>,
    /// Whether to add punctuation. Support varies by model (e.g., Deepgram, AssemblyAI). Defaults to `true`.
    /// Default: `true`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub punctuation: OptionalField<bool>,
    /// Enable speaker diarization. Defaults to `false`.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub diarization: OptionalField<bool>,
    /// Optional text prompt to guide the transcription model. Support varies (e.g., OpenAI).
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub initial_prompt: OptionalField<String>,
    /// Controls output randomness for supported models (e.g., OpenAI). Value between 0 and 1.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): minimum >= 0; maximum <= 1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub temperature: OptionalField<f64>,
    /// Hint for the number of expected speakers for diarization (e.g., RevAI, Deepgram).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): minimum >= 1; maximum <= 10.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub speakers_expected: OptionalField<i64>,
    /// List of custom words/phrases to improve recognition (e.g., Deepgram, AssemblyAI).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub custom_vocabulary: OptionalField<Vec<String>>,
}

impl BaseTranscriptionConfiguration {
    /// Server-side request validation (companion §9): structural checks stay in Serde decode; these enforce the D-§2
    /// bucket-2 constraints. Client decoding stays lenient.
    pub fn validate_request(
        &self,
    ) -> ::std::result::Result<(), ::openapi_support::validation::Violation> {
        if let OptionalField::Present(value) = &self.ruleset_id {
            ::openapi_support::validation::validate_format_string(value, "uuid")
                .map_err(|error| error.at_field("ruleset_id"))?;
        }
        if let OptionalField::Present(value) = &self.temperature {
            ::openapi_support::validation::validate_number(
                *value,
                Some((0.0, false)),
                Some((1.0, false)),
                None,
            )
            .map_err(|error| error.at_field("temperature"))?;
        }
        if let OptionalField::Present(value) = &self.speakers_expected {
            ::openapi_support::validation::validate_number(
                *value as f64,
                Some((1.0, false)),
                Some((10.0, false)),
                None,
            )
            .map_err(|error| error.at_field("speakers_expected"))?;
        }
        if let OptionalField::Present(value) = &self.custom_vocabulary {
            ::openapi_support::validation::require_unique_strings(value.iter())
                .map_err(|error| error.at_field("custom_vocabulary"))?;
        }
        Ok(())
    }
}

/// The language code of the audio file, typically in ISO 639-1 format.
/// Specifying the correct language improves transcription accuracy and speed.
/// The special value `auto` can be used to request automatic language detection, if supported by the selected model.
/// If omitted, the default language is English (`en`).
///
/// Default: `"en"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranscriptLanguageCode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en_au")]
    EnAu,
    #[serde(rename = "en_uk")]
    EnUk,
    #[serde(rename = "en_us")]
    EnUs,
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bo")]
    Bo,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "ha")]
    Ha,
    #[serde(rename = "haw")]
    Haw,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "jw")]
    Jw,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "lo")]
    Lo,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "si")]
    Si,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "su")]
    Su,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tl")]
    Tl,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "yo")]
    Yo,
    #[serde(rename = "zh")]
    Zh,
}

/// Specifies the desired format of the transcription output.
/// - `text`: Plain text containing the full transcription.
/// - `json_text`: A simple JSON object containing the transcription ID and the full text (`TranscriptionOnlyText` schema).
/// - `json`: A detailed JSON object including segments, timestamps (based on `timestamp_granularity`), language, and potentially speaker labels and provider metadata (`TranscriptionDetailed` schema).
/// - `srt`: SubRip subtitle format (returned as plain text).
/// - `vtt`: WebVTT subtitle format (returned as plain text).
///
/// Default: `"text"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranscriptOutputFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_text")]
    JsonText,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "vtt")]
    Vtt,
}

/// Configuration options for transcribing audio specified by a remote URL via the `/transcribe-remote` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteTranscriptionConfiguration {
    /// An opaque Speech-to-Text model identifier in `provider.model_name` form. Obtain currently available values from the `/speech-to-text-models` endpoint.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub model: OpenTranscriptionModelIdentifier,
    /// The language code of the audio file, typically in ISO 639-1 format.
    /// Specifying the correct language improves transcription accuracy and speed.
    /// The special value `auto` can be used to request automatic language detection, if supported by the selected model.
    /// If omitted, the default language is English (`en`).
    ///
    /// Default: `"en"`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub language: OptionalField<TranscriptLanguageCode>,
    /// Specifies the desired format of the transcription output.
    /// - `text`: Plain text containing the full transcription.
    /// - `json_text`: A simple JSON object containing the transcription ID and the full text (`TranscriptionOnlyText` schema).
    /// - `json`: A detailed JSON object including segments, timestamps (based on `timestamp_granularity`), language, and potentially speaker labels and provider metadata (`TranscriptionDetailed` schema).
    /// - `srt`: SubRip subtitle format (returned as plain text).
    /// - `vtt`: WebVTT subtitle format (returned as plain text).
    ///
    /// Default: `"text"`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub output_format: OptionalField<TranscriptOutputFormat>,
    /// The unique identifier (UUID) of a pre-defined replacement ruleset to apply to the final transcription text.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `uuid`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub ruleset_id: OptionalField<String>,
    /// Whether to add punctuation. Support varies by model (e.g., Deepgram, AssemblyAI). Defaults to `true`.
    /// Default: `true`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub punctuation: OptionalField<bool>,
    /// Enable speaker diarization. Defaults to `false`.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub diarization: OptionalField<bool>,
    /// Optional text prompt to guide the transcription model. Support varies (e.g., OpenAI).
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub initial_prompt: OptionalField<String>,
    /// Controls output randomness for supported models (e.g., OpenAI). Value between 0 and 1.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): minimum >= 0; maximum <= 1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub temperature: OptionalField<f64>,
    /// Hint for the number of expected speakers for diarization (e.g., RevAI, Deepgram).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): minimum >= 1; maximum <= 10.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub speakers_expected: OptionalField<i64>,
    /// List of custom words/phrases to improve recognition (e.g., Deepgram, AssemblyAI).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub custom_vocabulary: OptionalField<Vec<String>>,
    /// The publicly accessible URL of the audio file to transcribe. The API server must be able to fetch the audio from this URL.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `uri`; examples x1.
    pub file_url: String,
    /// An array of replacement rules to be applied directly to this transcription request, in order. This allows defining rules inline instead of (or in addition to) using a pre-saved `ruleset_id`.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub replacement_ruleset: OptionalField<Vec<ReplacementRule>>,
}

impl RemoteTranscriptionConfiguration {
    /// Server-side request validation (companion §9): structural checks stay in Serde decode; these enforce the D-§2
    /// bucket-2 constraints. Client decoding stays lenient.
    pub fn validate_request(
        &self,
    ) -> ::std::result::Result<(), ::openapi_support::validation::Violation> {
        if let OptionalField::Present(value) = &self.ruleset_id {
            ::openapi_support::validation::validate_format_string(value, "uuid")
                .map_err(|error| error.at_field("ruleset_id"))?;
        }
        if let OptionalField::Present(value) = &self.temperature {
            ::openapi_support::validation::validate_number(
                *value,
                Some((0.0, false)),
                Some((1.0, false)),
                None,
            )
            .map_err(|error| error.at_field("temperature"))?;
        }
        if let OptionalField::Present(value) = &self.speakers_expected {
            ::openapi_support::validation::validate_number(
                *value as f64,
                Some((1.0, false)),
                Some((10.0, false)),
                None,
            )
            .map_err(|error| error.at_field("speakers_expected"))?;
        }
        if let OptionalField::Present(value) = &self.custom_vocabulary {
            ::openapi_support::validation::require_unique_strings(value.iter())
                .map_err(|error| error.at_field("custom_vocabulary"))?;
        }
        ::openapi_support::validation::validate_format_string(&self.file_url, "uri")
            .map_err(|error| error.at_field("file_url"))?;
        Ok(())
    }
}

/// Defines a single rule for finding and replacing text in a transcription. Use one of the specific rule types (`ExactRule`, `RegexRule`, `RegexGroupRule`). The `kind` property acts as a discriminator.
/// Proven mutually exclusive branches (companion §4.2): exclusivity was proven statically, so derive-based untagged decoding preserves exactly-one validation.
/// Discriminator (routing hint only; inspect-select-validate decode arrives in a later package): property `kind`.
/// Mapping: exact -> ExactRule, regex -> RegexRule, regex_group -> RegexGroupRule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplacementRule {
    ExactRule(ExactRule),
    RegexRule(RegexRule),
    RegexGroupRule(RegexGroupRule),
}

/// Discriminator field identifying the rule type as 'exact'.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExactRuleKind {
    #[serde(rename = "exact")]
    Exact,
}

/// Defines a replacement rule based on finding an exact string match.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExactRule {
    /// Discriminator field identifying the rule type as 'exact'.
    pub kind: ExactRuleKind,
    /// The exact text string to search for within the transcription.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub search: String,
    /// The text string to replace the found 'search' text with.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub replacement: String,
    /// If true, the search will match only if the case is identical. If false (default), the search ignores case.
    /// Default: `false`.
    #[serde(rename = "caseSensitive")]
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub case_sensitive: OptionalField<bool>,
}

/// Discriminator field identifying the rule type as 'regex'.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegexRuleKind {
    #[serde(rename = "regex")]
    Regex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegexRuleFlagsItem {
    #[serde(rename = "i")]
    I,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "u")]
    U,
}

/// Defines a replacement rule based on matching a regular expression pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexRule {
    /// Discriminator field identifying the rule type as 'regex'.
    pub kind: RegexRuleKind,
    /// The regular expression pattern to search for. Uses standard regex syntax (implementation specific, often PCRE-like). Remember to escape special characters if needed (e.g., `\\.` for a literal dot).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `regex`; examples x1.
    pub pattern: String,
    /// The replacement text. Can include backreferences to capture groups from the pattern, like `$1`, `$2`, etc. A literal `$` should be escaped (e.g., `$$`).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub replacement: String,
    /// An array of flags to modify the regex behavior (e.g., 'i' for case-insensitivity).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub flags: OptionalField<Vec<RegexRuleFlagsItem>>,
}

/// Discriminator field identifying the rule type as 'regex_group'.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegexGroupRuleKind {
    #[serde(rename = "regex_group")]
    RegexGroup,
}

/// An object where keys are capture group numbers (as strings, e.g., "1", "2") and values are the respective replacement strings for those groups. Groups not listed are kept as matched. The entire match is reconstructed using these replacements.
/// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexGroupRuleGroupReplacements {
    #[serde(flatten)]
    pub additional: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegexGroupRuleFlagsItem {
    #[serde(rename = "i")]
    I,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "u")]
    U,
}

/// Defines a replacement rule that uses regex capture groups to apply different replacements to different parts of the matched text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexGroupRule {
    /// Discriminator field identifying the rule type as 'regex_group'.
    pub kind: RegexGroupRuleKind,
    /// The regular expression pattern containing capture groups `(...)`. The entire pattern must match for replacements to occur.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `regex`; examples x1.
    pub pattern: String,
    /// An object where keys are capture group numbers (as strings, e.g., "1", "2") and values are the respective replacement strings for those groups. Groups not listed are kept as matched. The entire match is reconstructed using these replacements.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(rename = "groupReplacements")]
    pub group_replacements: RegexGroupRuleGroupReplacements,
    /// An array of flags to modify the regex behavior.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub flags: OptionalField<Vec<RegexGroupRuleFlagsItem>>,
}

/// A simplified JSON response format containing only the transcription ID and the full transcribed text. Returned when `output_format` is `json_text`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionOnlyText {
    /// A unique identifier for the transcription job/request.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub id: String,
    /// The full transcribed text as a single string.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub text: String,
}

/// A detailed JSON response format containing the full text, detected language, duration, individual timed segments, and potentially speaker labels and provider-specific metadata. Returned when `output_format` is `json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionDetailed {
    /// A unique identifier for the transcription job/request.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub id: String,
    /// The full transcribed text as a single string.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub text: String,
    /// The detected or specified language of the audio (ISO 639-1 code).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub language: OptionalField<String>,
    /// An array of transcribed segments, providing time-coded chunks of the transcription. May include speaker labels if diarization was enabled.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub segments: OptionalField<Vec<TranscriptionSegment>>,
    /// An array of transcribed words, providing time-coded chunks of the transcription. May include speaker labels if diarization was enabled.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub words: OptionalField<Vec<TranscriptionWord>>,
}

/// Represents a time-coded segment of the transcription, typically corresponding to a phrase, sentence, or speaker turn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// The start time of the segment in seconds from the beginning of the audio.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub start: OptionalField<f64>,
    /// The end time of the segment in seconds from the beginning of the audio.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub end: OptionalField<f64>,
    /// The transcribed text content of this segment.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub text: OptionalField<String>,
    /// An identifier for the speaker of this segment, present if diarization was enabled and successful.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub speaker: OptionalField<String>,
    /// The model's confidence score for the transcription of this segment, typically between 0 and 1 (if provided by the model).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub confidence: OptionalField<f64>,
}

/// Represents a word in the transcription, providing time-coded chunks of the transcription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionWord {
    /// The start time of the word in seconds from the beginning of the audio.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    pub start: f64,
    /// The end time of the word in seconds from the beginning of the audio.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    pub end: f64,
    /// The transcribed word.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub word: String,
    /// An identifier for the speaker of this word, present if diarization was enabled and successful.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub speaker: OptionalField<String>,
    /// The model's confidence score for the transcription of this word, typically between 0 and 1 (if provided by the model).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default, skip_serializing_if = "openapi_support::optional::is_absent")]
    pub confidence: OptionalField<f64>,
}

/// Represents the JSON structure returned when a JSON-based `output_format` (`json` or `json_text`) is requested. It can be either a detailed structure or a simple text-only structure.
/// Raw/value fallback carrying retained validation metadata (companion §4.2, DECISIONS.md D-impl-oneoffallback): mutual exclusivity of the oneOf branches could not be proven statically (unproven-one-of); exactly-one semantics stay exact at the JSON level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionResponseFallback(pub serde_json::Value);

/// The primary type or training domain of the model. Helps identify suitability for different audio types.
/// Nullable: instances may be JSON `null`; reference sites wrap this type in `Option<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpeechToTextModelModelType {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "phone_call")]
    PhoneCall,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "command_and_search")]
    CommandAndSearch,
    #[serde(rename = "medical")]
    Medical,
    #[serde(rename = "legal")]
    Legal,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "meeting")]
    Meeting,
}

/// A general indication of the model's expected accuracy level relative to other models. Not a guaranteed metric.
/// Nullable: instances may be JSON `null`; reference sites wrap this type in `Option<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpeechToTextModelAccuracyTier {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "enhanced")]
    Enhanced,
    #[serde(rename = "premium")]
    Premium,
}

/// Describes an available speech-to-text model, its provider, capabilities, and characteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechToTextModel {
    /// An opaque Speech-to-Text model identifier in `provider.model_name` form. Obtain currently available values from the `/speech-to-text-models` endpoint.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub id: OpenTranscriptionModelIdentifier,
    /// A user-friendly name for the model.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub display_name: String,
    /// The identifier for the underlying Speech-to-Text service provider (e.g., 'openai', 'deepgram').
    pub provider: TranscriptionProvider,
    /// A brief description of the model, its intended use case, or version notes.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub description: Option<String>,
    /// The cost per second of audio processed in USD.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default)]
    pub cost_per_second_usd: Option<f64>,
    /// Indicates whether the model is currently available for use.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    /// Default: `true`.
    pub is_available: bool,
    /// A list of language codes (preferably BCP 47, e.g., "en-US", "en-GB", "es-ES") supported by this model. May include `auto` if automatic language detection is supported across multiple languages within a single audio file.
    ///
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub supported_languages: Option<Vec<String>>,
    /// Indicates whether the model generally supports automatic punctuation insertion.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub punctuation: Option<bool>,
    /// Indicates whether the model generally supports speaker diarization (identifying different speakers).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub diarization: Option<bool>,
    /// Indicates whether the model can be used for real-time streaming transcription via a WebSocket connection (if offered by Speechall).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub streamable: Option<bool>,
    /// An approximate measure of processing speed for batch processing. Defined as (audio duration) / (processing time). A higher value means faster processing (e.g., RTF=2 means it processes 1 second of audio in 0.5 seconds). May not be available for all models or streaming scenarios.
    ///
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default)]
    pub real_time_factor: Option<f64>,
    /// The maximum duration of a single audio file (in seconds) that the model can reliably process in one request. May vary by provider or plan.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `double`; examples x1.
    #[serde(default)]
    pub max_duration_seconds: Option<f64>,
    /// The maximum size of a single audio file (in bytes) that can be uploaded for processing by this model. May vary by provider or plan.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `int64`; examples x1.
    #[serde(default)]
    pub max_file_size_bytes: Option<i64>,
    /// The specific version identifier for the model.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub version: Option<String>,
    /// The date when this specific version of the model was released or last updated.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `date`; examples x1.
    #[serde(default)]
    pub release_date: Option<String>,
    /// The primary type or training domain of the model. Helps identify suitability for different audio types.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub model_type: Option<SpeechToTextModelModelType>,
    /// A general indication of the model's expected accuracy level relative to other models. Not a guaranteed metric.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub accuracy_tier: Option<SpeechToTextModelAccuracyTier>,
    /// A list of audio encodings that this model supports or is optimized for (e.g., LINEAR16, FLAC, MP3, Opus).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub supported_audio_encodings: Option<Vec<String>>,
    /// A list of audio sample rates (in Hz) that this model supports or is optimized for.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub supported_sample_rates: Option<Vec<i64>>,
    /// Indicates whether the model can provide speaker labels for the transcription.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub speaker_labels: Option<bool>,
    /// Indicates whether the model can provide timestamps for individual words.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub word_timestamps: Option<bool>,
    /// Indicates whether the model provides confidence scores for the transcription or individual words.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub confidence_scores: Option<bool>,
    /// Indicates whether the model supports automatic language detection for input audio.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub language_detection: Option<bool>,
    /// Indicates if the model can leverage a custom vocabulary or language model adaptation.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub custom_vocabulary_support: Option<bool>,
    /// Indicates if the model supports filtering or masking of profanity.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub profanity_filtering: Option<bool>,
    /// Indicates if the model supports noise reduction.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub noise_reduction: Option<bool>,
    /// Indicates whether the model supports SRT subtitle format output.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    /// Default: `false`.
    pub supports_srt: bool,
    /// Indicates whether the model supports VTT subtitle format output.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    /// Default: `false`.
    pub supports_vtt: bool,
    /// Indicates whether the model supports voice activity detection (VAD) to identify speech segments.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    #[serde(default)]
    pub voice_activity_detection: Option<bool>,
}

/// Standard structure for error responses. May include additional properties depending on the error type.
/// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// A human-readable message describing the error.
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateReplacementRulesetRequestBody {
    /// A user-defined name for this ruleset for easier identification.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): examples x1.
    pub name: String,
    /// An ordered array of replacement rules. Rules are applied in the order they appear in this list. See the `ReplacementRule` schema for different rule types (exact, regex, regex_group).
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): minItems >= 1.
    pub rules: Vec<ReplacementRule>,
}

impl CreateReplacementRulesetRequestBody {
    /// Server-side request validation (companion §9): structural checks stay in Serde decode; these enforce the D-§2
    /// bucket-2 constraints. Client decoding stays lenient.
    pub fn validate_request(
        &self,
    ) -> ::std::result::Result<(), ::openapi_support::validation::Violation> {
        ::openapi_support::validation::validate_array_len(
            self.rules.len(),
            &::openapi_support::validation::ArrayConstraints {
                min_items: Some(1),
                max_items: None,
            },
        )
        .map_err(|error| error.at_field("rules"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateReplacementRulesetResponseBody {
    /// The unique identifier (UUID) generated for this ruleset. Use this ID in the `ruleset_id` parameter of transcription requests.
    /// Constraints (enforced by generated routers on server requests, companion §9; lenient on client decode): format `uuid`; examples x1.
    pub id: String,
}

impl CreateReplacementRulesetResponseBody {
    /// Server-side request validation (companion §9): structural checks stay in Serde decode; these enforce the D-§2
    /// bucket-2 constraints. Client decoding stays lenient.
    pub fn validate_request(
        &self,
    ) -> ::std::result::Result<(), ::openapi_support::validation::Violation> {
        ::openapi_support::validation::validate_format_string(&self.id, "uuid")
            .map_err(|error| error.at_field("id"))?;
        Ok(())
    }
}
