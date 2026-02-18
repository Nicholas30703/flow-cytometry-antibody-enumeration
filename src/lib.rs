use serde::Serialize;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Panel {
    name: String,
    antibodies: Vec<String>,
}

#[derive(Serialize)]
struct EnumerationResult {
    unique: Vec<String>,
    count: usize,
}

fn build_panels() -> Vec<Panel> {
    vec![
        Panel {
            name: "Screening Tube".into(),
            antibodies: vec![
                "CD3".into(),
                "CD4".into(),
                "CD5".into(),
                "CD7".into(),
                "CD8".into(),
                "CD13".into(),
                "CD16".into(),
                "CD19".into(),
                "CD34".into(),
                "CD45".into(),
                "CD56".into(),
                "kappa".into(),
                "lambda".into(),
            ],
        },
        Panel {
            name: "Standard Tube 1 (T-cells)".into(),
            antibodies: vec![
                "CD2".into(),
                "CD3".into(),
                "CD4".into(),
                "CD5".into(),
                "CD7".into(),
                "CD8".into(),
                "CD19".into(),
                "CD38".into(),
                "CD45".into(),
                "CD56".into(),
            ],
        },
        Panel {
            name: "Standard Tube 2 (B-cells)".into(),
            antibodies: vec![
                "CD5".into(),
                "CD10".into(),
                "CD11c".into(),
                "CD19".into(),
                "CD20".into(),
                "CD23".into(),
                "CD45".into(),
                "CD200".into(),
                "kappa".into(),
                "lambda".into(),
            ],
        },
        Panel {
            name: "Standard Tube 3 (Myeloid)".into(),
            antibodies: vec![
                "CD13".into(),
                "CD14".into(),
                "CD16".into(),
                "CD19".into(),
                "CD33".into(),
                "CD34".into(),
                "CD45".into(),
                "CD64".into(),
                "CD117".into(),
                "HLA-DR".into(),
            ],
        },
        Panel {
            name: "Plasma Cell".into(),
            antibodies: vec![
                "CD19".into(),
                "CD20".into(),
                "CD38".into(),
                "CD45".into(),
                "CD56".into(),
                "CD81".into(),
                "CD117".into(),
                "CD138".into(),
                "kappa (cytoplasmic)".into(),
                "lambda (cytoplasmic)".into(),
            ],
        },
        Panel {
            name: "Acute Leukemia".into(),
            antibodies: vec![
                "CD3 (cytoplasmic)".into(),
                "CD7".into(),
                "CD13".into(),
                "CD19".into(),
                "CD22 (cytoplasmic)".into(),
                "CD33".into(),
                "CD34".into(),
                "CD45".into(),
                "Myeloperoxidase (cytoplasmic)".into(),
                "TdT (cytoplasmic)".into(),
            ],
        },
        Panel {
            name: "Hairy Cell".into(),
            antibodies: vec![
                "CD11c".into(),
                "CD19".into(),
                "CD20".into(),
                "CD22".into(),
                "CD25".into(),
                "CD45".into(),
                "CD79b".into(),
                "CD103".into(),
                "CD123".into(),
                "CD200".into(),
            ],
        },
        Panel {
            name: "LGL with TRBC1".into(),
            antibodies: vec![
                "CD3".into(),
                "CD4".into(),
                "CD7".into(),
                "CD8".into(),
                "CD16".into(),
                "CD45".into(),
                "CD56".into(),
                "CD57".into(),
                "TCR-D/G".into(),
                "TRBC1".into(),
            ],
        },
        Panel {
            name: "6-Color Tube 2".into(),
            antibodies: vec![
                "CD10".into(),
                "CD19".into(),
                "CD45".into(),
                "FMC7".into(),
                "kappa (cytoplasmic)".into(),
                "lambda (cytoplasmic)".into(),
            ],
        },
        Panel {
            name: "6-Color BAL".into(),
            antibodies: vec![
                "CD3".into(),
                "CD4".into(),
                "CD8".into(),
                "CD19".into(),
                "CD45".into(),
                "HLA-DR".into(),
            ],
        },
        Panel {
            name: "6-Color Limited Lymph".into(),
            antibodies: vec![
                "CD5".into(),
                "CD10".into(),
                "CD19".into(),
                "CD45".into(),
                "kappa (cytoplasmic)".into(),
                "lambda (cytoplasmic)".into(),
            ],
        },
    ]
}

#[wasm_bindgen]
pub fn get_panels() -> JsValue {
    let mut panels = build_panels();
    for panel in &mut panels {
        panel.antibodies.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));
    }
    serde_wasm_bindgen::to_value(&panels).unwrap_or(JsValue::NULL)
}

/// Build a sort key that zero-pads single/double-digit CD numbers to 3 digits
/// so that CD3 sorts before CD10 and CD10 sorts before CD103.
fn sort_key(s: &str) -> String {
    let lower = s.to_lowercase();
    if lower.starts_with("cd") {
        let rest = &lower[2..];
        let digit_end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
        if digit_end > 0 {
            let num: &str = &rest[..digit_end];
            let suffix: &str = &rest[digit_end..];
            return format!("cd{:0>3}{}", num, suffix);
        }
    }
    lower
}

#[wasm_bindgen]
pub fn enumerate_antibodies(selected_json: &str) -> JsValue {
    let selected: Vec<String> = serde_json::from_str(selected_json).unwrap_or_default();
    let unique: HashSet<String> = selected.into_iter().collect();
    let mut sorted: Vec<String> = unique.iter().cloned().collect();
    sorted.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));
    let count = sorted
        .iter()
        .filter(|ab| {
            if let Some(base) = ab.strip_suffix(" (cytoplasmic)") {
                !unique.contains(base)
            } else {
                true
            }
        })
        .count();
    let result = EnumerationResult {
        count,
        unique: sorted,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}
