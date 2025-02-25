use mistralrs_core::NormalLoaderType;
use pyo3::pyclass;

#[pyclass]
#[derive(Debug, Clone)]
pub enum Architecture {
    Mistral,
    Gemma,
    Mixtral,
    Llama,
    Phi2,
}

impl From<Architecture> for NormalLoaderType {
    fn from(value: Architecture) -> Self {
        match value {
            Architecture::Gemma => Self::Gemma,
            Architecture::Llama => Self::Llama,
            Architecture::Mistral => Self::Mistral,
            Architecture::Mixtral => Self::Mixtral,
            Architecture::Phi2 => Self::Phi2,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Which {
    Plain {
        model_id: String,
        tokenizer_json: Option<String>,
        repeat_last_n: Option<usize>,
        arch: Architecture,
    },

    XLora {
        model_id: Option<String>,
        tokenizer_json: Option<String>,
        xlora_model_id: String,
        repeat_last_n: Option<usize>,
        order: String,
        tgt_non_granular_index: Option<usize>,
        arch: Architecture,
    },

    Lora {
        model_id: Option<String>,
        tokenizer_json: Option<String>,
        adapters_model_id: String,
        repeat_last_n: Option<usize>,
        order: String,
        arch: Architecture,
    },

    #[allow(clippy::upper_case_acronyms)]
    GGUF {
        tok_model_id: String,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
    },

    XLoraGGUF {
        tok_model_id: Option<String>,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
        xlora_model_id: String,
        order: String,
        tgt_non_granular_index: Option<usize>,
    },

    LoraGGUF {
        tok_model_id: Option<String>,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
        adapters_model_id: String,
        order: String,
        tgt_non_granular_index: Option<usize>,
    },

    #[allow(clippy::upper_case_acronyms)]
    GGML {
        tok_model_id: String,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
        gqa: Option<usize>,
    },

    XLoraGGML {
        tok_model_id: Option<String>,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
        xlora_model_id: String,
        order: String,
        tgt_non_granular_index: Option<usize>,
        gqa: Option<usize>,
    },

    LoraGGML {
        tok_model_id: Option<String>,
        tokenizer_json: Option<String>,
        quantized_model_id: String,
        quantized_filename: String,
        repeat_last_n: Option<usize>,
        adapters_model_id: String,
        order: String,
        tgt_non_granular_index: Option<usize>,
        gqa: Option<usize>,
    },
}
