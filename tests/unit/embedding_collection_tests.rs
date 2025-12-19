// Tests for embedding collection from files (T.REQLIXF.3)
// Covers edge cases in collect_embeddings function

use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for collect_embeddings edge cases
// =============================================================================

#[test]
fn test_collect_embeddings_skips_agents_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#,
    );

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    // Should only have G.C.1, not embedding from AGENTS.md
    assert_eq!(embeddings.len(), 1);
    assert!(embeddings.contains_key("G.C.1"));
}

#[test]
fn test_collect_embeddings_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.
"#,
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        r#"# Chapter

## T.C.1: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#,
    );

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 2);
    assert!(embeddings.contains_key("G.C.1"));
    assert!(embeddings.contains_key("T.C.1"));
}

#[test]
fn test_collect_embeddings_skips_non_md_files() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#,
    );
    // Create a non-md file
    std::fs::write(req_dir.join("test.txt"), "<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->").unwrap();

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 1);
    assert!(embeddings.contains_key("G.C.1"));
}

#[test]
fn test_collect_embeddings_handles_duplicate_indices() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Same index in different files (should overwrite)
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.
"#,
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        r#"# Chapter

## G.C.1: Duplicate
<!--embedding:paraphrase-MiniLM-L3-v2:different_vector-->

Content2.
"#,
    );

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    // HashMap will overwrite, so should have only one
    assert_eq!(embeddings.len(), 1);
    assert!(embeddings.contains_key("G.C.1"));
}

#[test]
fn test_collect_embeddings_no_heading_before_embedding() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Embedding without preceding heading
    let content = r#"# Chapter

<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Some text.

## G.C.1: Test

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    // Should not collect embedding without heading
    assert_eq!(embeddings.len(), 0);
}

#[test]
fn test_collect_embeddings_heading_in_different_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter1

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

# Chapter2

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 2);
    assert!(embeddings.contains_key("G.C.1"));
    assert!(embeddings.contains_key("G.C.2"));
}

#[test]
fn test_collect_embeddings_skips_invalid_base64() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Valid
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Invalid
<!--embedding:paraphrase-MiniLM-L3-v2:invalid!!!-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 1);
    assert!(embeddings.contains_key("G.C.1"));
    assert!(!embeddings.contains_key("G.C.2"));
}

#[test]
fn test_collect_embeddings_handles_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "");

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 0);
}

#[test]
fn test_collect_embeddings_handles_file_with_only_headings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 0);
}

#[test]
fn test_collect_embeddings_handles_whitespace_around_embedding() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
  <!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->  

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    // Regex should still match with whitespace
    assert_eq!(embeddings.len(), 1);
}

#[test]
fn test_collect_embeddings_multiple_embeddings_same_requirement() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Multiple embeddings for same requirement (should use last one)
    // Use valid base64 encoded embeddings (384 f32 values = 1536 bytes)
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();
    for i in 0..384 {
        let val1 = (i as f32) / 100.0;
        let val2 = (i as f32) / 200.0;
        first_vec.extend_from_slice(&val1.to_le_bytes());
        second_vec.extend_from_slice(&val2.to_le_bytes());
    }
    let first_encoded = BASE64_STD.encode(&first_vec);
    let second_encoded = BASE64_STD.encode(&second_vec);
    let content = format!(r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content.
"#, first_encoded, second_encoded);
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 1);
    // Should use the last embedding found
    assert!(embeddings.contains_key("G.C.1"));
}

#[test]
fn test_collect_embeddings_very_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=100 {
        content.push_str(&format!(
            "## G.C.{}: Requirement {}\n<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->\n\nContent {}.\n\n",
            i, i, i
        ));
    }
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 100);
}

#[test]
fn test_collect_embeddings_embedding_at_file_start() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Embedding at very start (after chapter heading)
    let content = r#"# Chapter
## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 1);
    assert!(embeddings.contains_key("G.C.1"));
}

#[test]
fn test_collect_embeddings_embedding_at_file_end() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 1);
}

#[test]
fn test_collect_embeddings_level1_heading_between() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter1

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

# Chapter2

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    assert_eq!(embeddings.len(), 2);
    // Each embedding should be associated with its own requirement
    assert!(embeddings.contains_key("G.C.1"));
    assert!(embeddings.contains_key("G.C.2"));
}

#[test]
fn test_collect_embeddings_ignores_model_name() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Different model names should all be collected (model name is ignored)
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:model1:dGVzdA==-->

Content1.

## G.C.2: Second
<!--embedding:model2:dGVzdA==-->

Content2.

## G.C.3: Third
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content3.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let embeddings = reqlix::collect_embeddings(&req_dir).unwrap();
    // All should be collected regardless of model name
    assert_eq!(embeddings.len(), 3);
}
