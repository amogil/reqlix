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

#[path = "unit/tool_get_version_tests.rs"]
mod tool_get_version_tests;
