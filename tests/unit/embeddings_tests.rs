// Tests for Embeddings functionality (G.R.11, G.R.12, G.R.13)
// Covers embedding encoding/decoding, comment parsing, similarity calculation

// =============================================================================
// Tests for embedding encoding/decoding (G.R.11)
// =============================================================================

#[test]
fn test_encode_embedding_valid() {
    let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let encoded = reqlix::encode_embedding(&embedding);
    assert!(!encoded.is_empty());
    // Base64 should be non-empty and contain valid characters
    assert!(encoded.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '='));
}

#[test]
fn test_decode_embedding_valid() {
    let embedding = vec![0.1, 0.2, 0.3];
    let encoded = reqlix::encode_embedding(&embedding);
    let decoded = reqlix::decode_embedding(&encoded).unwrap();
    assert_eq!(decoded.len(), 3);
    assert!((decoded[0] - 0.1).abs() < 0.0001);
    assert!((decoded[1] - 0.2).abs() < 0.0001);
    assert!((decoded[2] - 0.3).abs() < 0.0001);
}

#[test]
fn test_encode_decode_roundtrip() {
    let original = vec![0.0, -1.0, 1.0, 0.5, -0.5, 0.123456];
    let encoded = reqlix::encode_embedding(&original);
    let decoded = reqlix::decode_embedding(&encoded).unwrap();
    assert_eq!(decoded.len(), original.len());
    for (a, b) in original.iter().zip(decoded.iter()) {
        assert!((a - b).abs() < 0.0001, "Roundtrip failed: {} != {}", a, b);
    }
}

#[test]
fn test_decode_embedding_invalid_base64() {
    let result = reqlix::decode_embedding("invalid!!!base64");
    assert!(result.is_err());
}

#[test]
fn test_decode_embedding_wrong_length() {
    // Create invalid base64 that decodes to length not divisible by 4
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let invalid = BASE64_STD.encode([1u8, 2u8, 3u8]);
    let result = reqlix::decode_embedding(&invalid);
    assert!(result.is_err());
}

#[test]
fn test_encode_embedding_empty() {
    let embedding: Vec<f32> = vec![];
    let encoded = reqlix::encode_embedding(&embedding);
    assert_eq!(encoded, "");
}

#[test]
fn test_decode_embedding_empty() {
    let result = reqlix::decode_embedding("");
    assert!(result.is_err()); // Empty string is invalid
}

#[test]
fn test_encode_embedding_large() {
    let embedding: Vec<f32> = (0..1000).map(|i| i as f32 / 1000.0).collect();
    let encoded = reqlix::encode_embedding(&embedding);
    assert!(!encoded.is_empty());
    let decoded = reqlix::decode_embedding(&encoded).unwrap();
    assert_eq!(decoded.len(), 1000);
}

#[test]
fn test_encode_embedding_negative_values() {
    let embedding = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let encoded = reqlix::encode_embedding(&embedding);
    let decoded = reqlix::decode_embedding(&encoded).unwrap();
    assert_eq!(decoded.len(), 5);
    assert!((decoded[0] - (-1.0)).abs() < 0.0001);
}

#[test]
fn test_encode_embedding_extreme_values() {
    let embedding = vec![f32::MAX, f32::MIN, f32::EPSILON, -f32::EPSILON];
    let encoded = reqlix::encode_embedding(&embedding);
    let decoded = reqlix::decode_embedding(&encoded).unwrap();
    assert_eq!(decoded.len(), 4);
}

// =============================================================================
// Tests for embedding comment parsing (G.R.11, T.REQLIXF.3)
// =============================================================================

#[test]
fn test_parse_embedding_comment_valid() {
    let comment = "<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->";
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_some());
    let (model, vector) = result.unwrap();
    assert_eq!(model, "paraphrase-MiniLM-L3-v2");
    assert_eq!(vector, "dGVzdA==");
}

#[test]
fn test_parse_embedding_comment_different_model_name() {
    let comment = "<!--embedding:some-other-model:dGVzdA==-->";
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_some());
    let (model, _) = result.unwrap();
    assert_eq!(model, "some-other-model");
}

#[test]
fn test_parse_embedding_comment_invalid_format() {
    let comment = "<!--embedding:dGVzdA==-->"; // Missing model name
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_none());
}

#[test]
fn test_parse_embedding_comment_missing_colon() {
    let comment = "<!--embedding:paraphrase-MiniLM-L3-v2dGVzdA==-->"; // Missing colon
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_none());
}

#[test]
fn test_parse_embedding_comment_not_html_comment() {
    let comment = "embedding:paraphrase-MiniLM-L3-v2:dGVzdA=="; // Not HTML comment
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_none());
}

#[test]
fn test_parse_embedding_comment_with_whitespace() {
    let comment = "  <!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->  ";
    let result = reqlix::parse_embedding_comment(comment.trim());
    assert!(result.is_some());
}

#[test]
fn test_parse_embedding_comment_empty_model_name() {
    let comment = "<!--embedding::dGVzdA==-->"; // Empty model name
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_some()); // Should still parse, model name can be empty
    let (model, _) = result.unwrap();
    assert_eq!(model, "");
}

#[test]
fn test_parse_embedding_comment_empty_vector() {
    let comment = "<!--embedding:paraphrase-MiniLM-L3-v2:-->"; // Empty vector
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_some());
    let (_, vector) = result.unwrap();
    assert_eq!(vector, "");
}

#[test]
fn test_parse_embedding_comment_special_chars_in_model() {
    let comment = "<!--embedding:model-name_v1.0:dGVzdA==-->";
    let result = reqlix::parse_embedding_comment(comment);
    assert!(result.is_some());
    let (model, _) = result.unwrap();
    assert_eq!(model, "model-name_v1.0");
}

#[test]
fn test_is_embedding_comment_valid() {
    assert!(reqlix::is_embedding_comment("<!--embedding:model:vector-->"));
    assert!(reqlix::is_embedding_comment("  <!--embedding:model:vector-->  "));
}

#[test]
fn test_is_embedding_comment_invalid() {
    assert!(!reqlix::is_embedding_comment("<!--not-embedding:model:vector-->"));
    assert!(!reqlix::is_embedding_comment("embedding:model:vector"));
    assert!(!reqlix::is_embedding_comment("<!--embedding:model:vector"));
}

// =============================================================================
// Tests for cosine similarity (T.REQLIXF.3)
// =============================================================================

#[test]
fn test_cosine_similarity_identical_vectors() {
    let vec = vec![0.5, 0.5, 0.5];
    let similarity = reqlix::cosine_similarity(&vec, &vec);
    assert!((similarity - 1.0).abs() < 0.0001, "Identical vectors should have similarity 1.0");
}

#[test]
fn test_cosine_similarity_orthogonal_vectors() {
    let vec1 = vec![1.0, 0.0, 0.0];
    let vec2 = vec![0.0, 1.0, 0.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert!((similarity - 0.0).abs() < 0.0001, "Orthogonal vectors should have similarity 0.0");
}

#[test]
fn test_cosine_similarity_opposite_vectors() {
    let vec1 = vec![1.0, 0.0, 0.0];
    let vec2 = vec![-1.0, 0.0, 0.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert!(similarity < 0.0, "Opposite vectors should have negative similarity");
}

#[test]
fn test_cosine_similarity_different_lengths() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![1.0, 2.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert_eq!(similarity, 0.0, "Different length vectors should return 0.0");
}

#[test]
fn test_cosine_similarity_zero_vectors() {
    let vec1 = vec![0.0, 0.0, 0.0];
    let vec2 = vec![1.0, 2.0, 3.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert_eq!(similarity, 0.0, "Zero vector should return 0.0");
}

#[test]
fn test_cosine_similarity_both_zero_vectors() {
    let vec = vec![0.0, 0.0, 0.0];
    let similarity = reqlix::cosine_similarity(&vec, &vec);
    assert_eq!(similarity, 0.0, "Both zero vectors should return 0.0");
}

#[test]
fn test_cosine_similarity_normalized_vectors() {
    // Normalized vectors (unit length)
    let vec1 = vec![1.0 / 3.0_f32.sqrt(), 1.0 / 3.0_f32.sqrt(), 1.0 / 3.0_f32.sqrt()];
    let vec2 = vec![1.0 / 2.0_f32.sqrt(), 1.0 / 2.0_f32.sqrt(), 0.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert!((-1.0..=1.0).contains(&similarity));
}

#[test]
fn test_cosine_similarity_range() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert!((-1.0..=1.0).contains(&similarity), "Similarity should be in [-1, 1] range");
}

#[test]
fn test_cosine_similarity_single_element() {
    let vec1 = vec![1.0];
    let vec2 = vec![2.0];
    let similarity = reqlix::cosine_similarity(&vec1, &vec2);
    assert!((similarity - 1.0).abs() < 0.0001, "Same direction single-element vectors should be 1.0");
}

// =============================================================================
// Tests for format_embedding_comment (G.R.11)
// =============================================================================

#[test]
fn test_format_embedding_comment_valid() {
    let embedding = vec![0.1, 0.2, 0.3];
    let comment = reqlix::format_embedding_comment("paraphrase-MiniLM-L3-v2", &embedding);
    assert!(comment.starts_with("<!--embedding:"));
    assert!(comment.ends_with("-->"));
    assert!(comment.contains("paraphrase-MiniLM-L3-v2"));
    assert!(comment.contains(':'));
}

#[test]
fn test_format_embedding_comment_empty_model() {
    let embedding = vec![0.1, 0.2];
    let comment = reqlix::format_embedding_comment("", &embedding);
    assert!(comment.contains("<!--embedding::"));
}

#[test]
fn test_format_embedding_comment_special_model_name() {
    let embedding = vec![0.1];
    let comment = reqlix::format_embedding_comment("model_v2.0-beta", &embedding);
    assert!(comment.contains("model_v2.0-beta"));
}
