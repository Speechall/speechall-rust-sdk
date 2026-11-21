use speechall::models::SpeechToTextModel;

#[test]
fn catalog_decodes_unknown_values() {
    let body = r#"[
        {
            "id": "futurecorp.model-x",
            "display_name": "FutureCorp Model X",
            "provider": "futurecorp",
            "is_available": true,
            "supports_srt": false,
            "supports_vtt": false,
            "model_type": "podcast",
            "accuracy_tier": "ultra"
        },
        {
            "id": "openai.whisper-1",
            "display_name": "Whisper",
            "provider": "openai",
            "is_available": true,
            "supports_srt": false,
            "supports_vtt": false,
            "model_type": null,
            "accuracy_tier": null
        }
    ]"#;

    let models: Vec<SpeechToTextModel> = serde_json::from_str(body).expect("decode catalog");
    let [future, known] = models.as_slice() else {
        panic!("expected two catalog entries");
    };

    assert_eq!(future.id, "futurecorp.model-x");
    assert_eq!(future.provider, "futurecorp");
    assert_eq!(future.model_type.as_deref(), Some("podcast"));
    assert_eq!(future.accuracy_tier.as_deref(), Some("ultra"));
    assert_eq!(known.provider, "openai");
    assert_eq!(known.model_type, None);
    assert_eq!(known.accuracy_tier, None);
}
