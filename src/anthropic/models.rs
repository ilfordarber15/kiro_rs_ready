//! Anthropic-compatible model registry.

use super::types::Model;

#[derive(Debug, Clone, Copy)]
struct ModelInfo {
    id: &'static str,
    display_name: &'static str,
    created: i64,
    max_tokens: i32,
    kiro_model: &'static str,
    context_window: i32,
}

const DEFAULT_CONTEXT_WINDOW: i32 = 200_000;
const LARGE_CONTEXT_WINDOW: i32 = 1_000_000;

const MODELS: &[ModelInfo] = &[
    ModelInfo {
        id: "claude-opus-4-8",
        display_name: "Claude Opus 4.8",
        created: 1_779_897_600,
        max_tokens: 128_000,
        kiro_model: "claude-opus-4.8",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-8-thinking",
        display_name: "Claude Opus 4.8 (Thinking)",
        created: 1_779_897_600,
        max_tokens: 128_000,
        kiro_model: "claude-opus-4.8",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-7",
        display_name: "Claude Opus 4.7",
        created: 1_776_276_000,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.7",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-7-thinking",
        display_name: "Claude Opus 4.7 (Thinking)",
        created: 1_776_276_000,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.7",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-6",
        display_name: "Claude Opus 4.6",
        created: 1_770_163_200,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.6",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-6-thinking",
        display_name: "Claude Opus 4.6 (Thinking)",
        created: 1_770_163_200,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.6",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-sonnet-4-6",
        display_name: "Claude Sonnet 4.6",
        created: 1_771_286_400,
        max_tokens: 64_000,
        kiro_model: "claude-sonnet-4.6",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-sonnet-4-6-thinking",
        display_name: "Claude Sonnet 4.6 (Thinking)",
        created: 1_771_286_400,
        max_tokens: 64_000,
        kiro_model: "claude-sonnet-4.6",
        context_window: LARGE_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-5-20251101",
        display_name: "Claude Opus 4.5",
        created: 1_763_942_400,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-opus-4-5-20251101-thinking",
        display_name: "Claude Opus 4.5 (Thinking)",
        created: 1_763_942_400,
        max_tokens: 64_000,
        kiro_model: "claude-opus-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-sonnet-4-5-20250929",
        display_name: "Claude Sonnet 4.5",
        created: 1_759_104_000,
        max_tokens: 64_000,
        kiro_model: "claude-sonnet-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-sonnet-4-5-20250929-thinking",
        display_name: "Claude Sonnet 4.5 (Thinking)",
        created: 1_759_104_000,
        max_tokens: 64_000,
        kiro_model: "claude-sonnet-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-haiku-4-5-20251001",
        display_name: "Claude Haiku 4.5",
        created: 1_760_486_400,
        max_tokens: 64_000,
        kiro_model: "claude-haiku-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
    ModelInfo {
        id: "claude-haiku-4-5-20251001-thinking",
        display_name: "Claude Haiku 4.5 (Thinking)",
        created: 1_760_486_400,
        max_tokens: 64_000,
        kiro_model: "claude-haiku-4.5",
        context_window: DEFAULT_CONTEXT_WINDOW,
    },
];

pub fn list_models() -> Vec<Model> {
    MODELS
        .iter()
        .map(|model| Model {
            id: model.id.to_string(),
            object: "model".to_string(),
            created: model.created,
            owned_by: "anthropic".to_string(),
            display_name: model.display_name.to_string(),
            model_type: "chat".to_string(),
            max_tokens: model.max_tokens,
        })
        .collect()
}

pub fn map_model(model: &str) -> Option<String> {
    find_model(model).map(|model| model.kiro_model.to_string())
}

pub fn context_window_size(model: &str) -> i32 {
    find_model(model)
        .map(|model| model.context_window)
        .unwrap_or(DEFAULT_CONTEXT_WINDOW)
}

fn find_model(model: &str) -> Option<ModelInfo> {
    let model_lower = model.to_lowercase();

    if let Some(exact) = MODELS.iter().find(|info| model_lower == info.id) {
        return Some(*exact);
    }

    if model_lower.contains("sonnet") {
        if model_lower.contains("4-6") || model_lower.contains("4.6") {
            return model_by_kiro_id("claude-sonnet-4.6");
        }
        return model_by_kiro_id("claude-sonnet-4.5");
    }

    if model_lower.contains("opus") {
        if model_lower.contains("4-8") || model_lower.contains("4.8") {
            return model_by_kiro_id("claude-opus-4.8");
        }
        if model_lower.contains("4-7") || model_lower.contains("4.7") {
            return model_by_kiro_id("claude-opus-4.7");
        }
        if model_lower.contains("4-5") || model_lower.contains("4.5") {
            return model_by_kiro_id("claude-opus-4.5");
        }
        return model_by_kiro_id("claude-opus-4.6");
    }

    if model_lower.contains("haiku") {
        return model_by_kiro_id("claude-haiku-4.5");
    }

    None
}

fn model_by_kiro_id(kiro_model: &str) -> Option<ModelInfo> {
    MODELS
        .iter()
        .find(|info| info.kiro_model == kiro_model && !info.id.ends_with("-thinking"))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listed_models_are_mappable() {
        for model in list_models() {
            assert!(
                map_model(&model.id).is_some(),
                "listed model should map to a Kiro model: {}",
                model.id
            );
        }
    }

    #[test]
    fn thinking_aliases_map_to_same_kiro_model() {
        for model in MODELS
            .iter()
            .filter(|model| model.id.ends_with("-thinking"))
        {
            let base_id = model.id.trim_end_matches("-thinking");
            assert_eq!(map_model(model.id), map_model(base_id), "{base_id}");
        }
    }

    #[test]
    fn generic_legacy_names_still_map() {
        assert_eq!(
            map_model("claude-sonnet-4-20250514"),
            Some("claude-sonnet-4.5".to_string())
        );
        assert_eq!(
            map_model("claude-opus-4-20250514"),
            Some("claude-opus-4.6".to_string())
        );
        assert_eq!(
            map_model("claude-3-5-sonnet-20241022"),
            Some("claude-sonnet-4.5".to_string())
        );
    }

    #[test]
    fn context_windows_follow_registry() {
        assert_eq!(context_window_size("claude-sonnet-4-6"), 1_000_000);
        assert_eq!(context_window_size("claude-opus-4-8-thinking"), 1_000_000);
        assert_eq!(context_window_size("claude-sonnet-4-5-20250929"), 200_000);
        assert_eq!(context_window_size("gpt-4"), 200_000);
    }
}
