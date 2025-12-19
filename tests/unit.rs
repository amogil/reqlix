// Common helper functions
#[path = "unit/common/mod.rs"]
mod common;

// Tests grouped by requirement chapters
#[path = "unit/parameter_constraints_tests.rs"]
mod parameter_constraints_tests;

#[path = "unit/requirements_storage_format_tests.rs"]
mod requirements_storage_format_tests;

#[path = "unit/configuration_tests.rs"]
mod configuration_tests;

// Tool-specific tests
#[path = "unit/tool_get_instructions_tests.rs"]
mod tool_get_instructions_tests;

#[path = "unit/tool_get_categories_tests.rs"]
mod tool_get_categories_tests;

#[path = "unit/tool_get_chapters_tests.rs"]
mod tool_get_chapters_tests;

#[path = "unit/tool_get_requirements_tests.rs"]
mod tool_get_requirements_tests;

#[path = "unit/tool_get_requirement_tests.rs"]
mod tool_get_requirement_tests;

#[path = "unit/tool_insert_requirement_tests.rs"]
mod tool_insert_requirement_tests;

#[path = "unit/tool_update_requirement_tests.rs"]
mod tool_update_requirement_tests;

#[path = "unit/tool_delete_requirement_tests.rs"]
mod tool_delete_requirement_tests;

#[path = "unit/tool_search_requirements_tests.rs"]
mod tool_search_requirements_tests;

#[path = "unit/tool_fuzzy_search_requirements_tests.rs"]
mod tool_fuzzy_search_requirements_tests;

#[path = "unit/tool_get_version_tests.rs"]
mod tool_get_version_tests;

// General requirements tests
#[path = "unit/general_requirements_tests.rs"]
mod general_requirements_tests;

// Embedding and fuzzy search tests
#[path = "unit/embeddings_tests.rs"]
mod embeddings_tests;

#[path = "unit/embedding_integration_tests.rs"]
mod embedding_integration_tests;

#[path = "unit/fuzzy_search_edge_cases_tests.rs"]
mod fuzzy_search_edge_cases_tests;

#[path = "unit/embedding_collection_tests.rs"]
mod embedding_collection_tests;

#[path = "unit/embedding_error_handling_tests.rs"]
mod embedding_error_handling_tests;

#[path = "unit/fuzzy_search_comprehensive_tests.rs"]
mod fuzzy_search_comprehensive_tests;

#[path = "unit/embedding_model_embedding_tests.rs"]
mod embedding_model_embedding_tests;
