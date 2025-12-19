// Embedding functionality for fuzzy search (G.R.11, G.R.13, T.REQLIXF.3)

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub const MODEL_NAME: &str = "paraphrase-MiniLM-L3-v2";

// Model components cache (G.R.13: lazy loading on first use, reused afterwards)
// Using Candle for embedding calculation as per G.R.13
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

// G.R.13: Model files embedded in binary at compile time
// Model files are embedded using include_bytes!/include_str! macros

// Embedded model files (G.R.13)
const MODEL_SAFETENSORS: &[u8] = include_bytes!("../models/model.safetensors");
const TOKENIZER_JSON: &str = include_str!("../models/tokenizer.json");
const CONFIG_JSON: &str = include_str!("../models/config.json");

struct ModelComponents {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    _temp_file: tempfile::NamedTempFile, // Keep temp file alive for tokenizer
}

// Global model cache (lazy initialization) - G.R.13: loaded lazily on first use, reused afterwards
static MODEL_CACHE: LazyLock<Mutex<Option<ModelComponents>>> = LazyLock::new(|| Mutex::new(None));

/// Initialize the embedding model (G.R.13: lazy loading on first use)
/// G.R.13: Model must be loaded from embedded data, not external files
fn init_model() -> Result<ModelComponents> {
    let device = Device::Cpu;
    
    // G.R.13: Load from embedded model files
    
    // Load tokenizer from embedded JSON string (tokenizers 0.20 API: requires file)
    // G.R.13: We use a temporary file to load the embedded tokenizer
    // Keep the temp file alive by storing it in ModelComponents
    let temp_file = tempfile::NamedTempFile::new()
        .context("Failed to create temporary file for tokenizer")?;
    std::fs::write(temp_file.path(), TOKENIZER_JSON)
        .context("Failed to write tokenizer JSON to temp file")?;
    let tokenizer = Tokenizer::from_file(temp_file.path())
        .map_err(|e| anyhow::anyhow!("Failed to load embedded tokenizer: {e}"))?;
    
    // Parse config from embedded JSON string
    let config: Config = serde_json::from_str(CONFIG_JSON)
        .context("Failed to parse embedded config")?;
    
    // Load model weights from embedded safetensors (Candle 0.9 API)
    let vb = VarBuilder::from_slice_safetensors(MODEL_SAFETENSORS, candle_core::DType::F32, &device)
        .context("Failed to load embedded model weights")?;
    let model = BertModel::load(vb, &config)
        .context("Failed to create BERT model")?;
    
    Ok(ModelComponents {
        model,
        tokenizer,
        device,
        _temp_file: temp_file, // Keep temp file alive
    })
}

/// Get or initialize the embedding model (G.R.13: lazy loading, singleton pattern)
/// Loads the model on first use and reuses it for all subsequent calls
fn get_model() -> Result<std::sync::MutexGuard<'static, Option<ModelComponents>>> {
    let mut cache = MODEL_CACHE.lock().unwrap();
    if cache.is_none() {
        *cache = Some(init_model()?);
    }
    Ok(cache)
}

/// Calculate embedding for requirement text (T.REQLIXI.6, T.REQLIXU.7, T.REQLIXF.3)
/// Uses paraphrase-MiniLM-L3-v2 model as per G.R.13
pub fn calculate_embedding(text: &str) -> Result<Vec<f32>> {
    let model_guard = get_model()?;
    let components = model_guard.as_ref().unwrap();
    
    // Tokenize input
    let encoding = components.tokenizer
        .encode(text, true)
        .map_err(|e| anyhow::anyhow!("Tokenization failed: {e}"))?;
    
    let token_ids: Vec<u32> = encoding.get_ids().to_vec();
    let token_ids_tensor = Tensor::new(&token_ids[..], &components.device)
        .context("Failed to create token IDs tensor")?
        .unsqueeze(0)?; // Add batch dimension
    
    // Generate embeddings (Candle 0.9 API: forward(input_ids, token_type_ids, attention_mask))
    // For BERT single sequence, token_type_ids should be zeros (not same as input_ids)
    let token_type_ids = Tensor::zeros((1, token_ids.len()), candle_core::DType::U32, &components.device)
        .context("Failed to create token type IDs tensor")?;
    let embeddings = components.model
        .forward(&token_ids_tensor, &token_type_ids, None)
        .context("Failed to generate embeddings")?;
    
    // Apply mean pooling (sentence embedding)
    let (_batch_size, seq_len, _hidden_size) = embeddings.dims3()
        .context("Invalid embedding dimensions")?;
    
    // Sum over sequence length (dim 1) and divide by sequence length
    let sum_embeddings = embeddings.sum(1)
        .context("Failed to sum embeddings")?;
    // Divide by sequence length to get mean - use affine method for scalar division
    let seq_len_f32 = seq_len as f32;
    let sentence_embedding = sum_embeddings.affine(1.0 / f64::from(seq_len_f32), 0.0)
        .context("Failed to normalize embeddings")?;
    
    // Extract as Vec<f32> - sentence_embedding is now (batch_size, hidden_size)
    let embedding_vec: Vec<f32> = sentence_embedding
        .to_vec2()
        .context("Failed to convert embedding to vector")?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty embedding"))?;
    
    // Normalize the embedding (L2 normalization)
    let norm: f32 = embedding_vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        Ok(embedding_vec.into_iter().map(|x| x / norm).collect())
    } else {
        Ok(embedding_vec)
    }
}

/// Encode embedding to base64 string for storage (G.R.11)
pub fn encode_embedding(embedding: &[f32]) -> String {
    // Convert f32 to bytes
    let bytes: Vec<u8> = embedding
        .iter()
        .flat_map(|&f| f.to_le_bytes().to_vec())
        .collect();
    STANDARD.encode(&bytes)
}

/// Decode embedding from base64 string (T.REQLIXF.3)
pub fn decode_embedding(encoded: &str) -> Result<Vec<f32>> {
    if encoded.is_empty() {
        return Err(anyhow::anyhow!("Empty embedding string"));
    }
    let bytes = STANDARD.decode(encoded)?;
    if bytes.len() % 4 != 0 {
        return Err(anyhow::anyhow!("Invalid embedding length"));
    }
    let mut embedding = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks(4) {
        let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        embedding.push(value);
    }
    Ok(embedding)
}

/// Format embedding comment (G.R.11)
pub fn format_embedding_comment(model_name: &str, embedding: &[f32]) -> String {
    let encoded = encode_embedding(embedding);
    format!("<!--embedding:{model_name}:{encoded}-->")
}

/// Parse embedding comment (T.REQLIXF.3)
pub fn parse_embedding_comment(comment: &str) -> Option<(String, String)> {
    // Allow empty model name and empty vector (model name can be empty, vector can be empty)
    let re = Regex::new(r"<!--embedding:([^:]*):([^>]*)-->").ok()?;
    let caps = re.captures(comment)?;
    let model_name = caps.get(1)?.as_str().to_string();
    let encoded_vector = caps.get(2)?.as_str().to_string();
    Some((model_name, encoded_vector))
}

/// Check if a line is an embedding comment (G.R.12)
pub fn is_embedding_comment(line: &str) -> bool {
    line.trim().starts_with("<!--embedding:") && line.trim().ends_with("-->")
}

/// Collect all embeddings from requirement files (T.REQLIXF.3)
pub fn collect_embeddings(
    requirements_dir: &std::path::PathBuf,
) -> Result<HashMap<String, Vec<f32>>> {
    let mut embeddings: HashMap<String, Vec<f32>> = HashMap::new();
    // Allow empty model name and empty vector (G.R.11: model name can be empty)
    let embedding_regex = Regex::new(r"<!--embedding:([^:]*):([^>]*)-->")?;
    
    // Iterate over all category files
    for entry in std::fs::read_dir(requirements_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() != Some(std::ffi::OsStr::new("md")) {
            continue;
        }
        if path.file_name() == Some(std::ffi::OsStr::new("AGENTS.md")) {
            continue;
        }
        
        let content = std::fs::read_to_string(&path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // Find embedding comments and their preceding requirement headings
        // T.REQLIXF.3: Ignore model name from comment, always use paraphrase-MiniLM-L3-v2 (G.R.11)
        for (i, line) in lines.iter().enumerate() {
            if let Some(caps) = embedding_regex.captures(line) {
                // Look backwards for the requirement heading
                for j in (0..i).rev() {
                    if let Some((index, _)) = crate::parsing::parse_level2_heading(lines[j]) {
                        // Extract only the vector (caps.get(2)), ignore model name (caps.get(1))
                        if let Some(encoded_vector) = caps.get(2) {
                            // If multiple embeddings for same requirement, use the last one (overwrite)
                            // T.REQLIXF.3 step 2: If decoding fails, silently ignore and treat requirement as having no embedding
                            match decode_embedding(encoded_vector.as_str()) {
                                Ok(embedding) => {
                                    embeddings.insert(index, embedding); // Overwrites previous if exists
                                }
                                Err(_) => {
                                    // T.REQLIXF.3 step 2: Skip invalid embeddings - requirement is treated as having no embedding
                                    // This requirement will be excluded from search results
                                    continue;
                                }
                            }
                        }
                        break; // Found heading, move to next embedding
                    }
                }
            }
        }
    }
    
    Ok(embeddings)
}

/// Calculate cosine similarity between two embeddings (T.REQLIXF.3)
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot_product / (norm_a * norm_b)
}
