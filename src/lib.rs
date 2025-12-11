// Main library module - re-exports and RequirementsServer struct

mod constants;
mod descriptions;
mod filesystem;
mod handlers;
mod helpers;
mod models;
mod params;
mod parsing;
mod response;
mod server;
mod validation;

// Re-export public types for external use
pub use models::{DeletedRequirement, RequirementFull, RequirementSummary};
pub use params::*;

// Re-export public functions for tests (module-level)
#[cfg(test)]
pub use filesystem::*;
#[cfg(test)]
pub use handlers::*;
#[cfg(test)]
pub use helpers::*;
#[cfg(test)]
pub use parsing::*;
#[cfg(test)]
pub use validation::*;

// =============================================================================
// Main server struct
// =============================================================================

#[derive(Debug, Clone, Default)]
pub struct RequirementsServer;

impl RequirementsServer {
    pub fn new() -> Self {
        Self
    }

    // Compatibility methods for tests - delegate to module functions
    pub fn validate_project_root(value: &str) -> Result<(), String> {
        validation::validate_project_root(value)
    }

    pub fn validate_operation_description(value: &str) -> Result<(), String> {
        validation::validate_operation_description(value)
    }

    pub fn validate_category(value: &str) -> Result<(), String> {
        validation::validate_category(value)
    }

    pub fn validate_chapter(value: &str) -> Result<(), String> {
        validation::validate_chapter(value)
    }

    pub fn validate_index(value: &str) -> Result<(), String> {
        validation::validate_index(value)
    }

    pub fn validate_text(value: &str) -> Result<(), String> {
        validation::validate_text(value)
    }

    pub fn validate_title(value: &str, required: bool) -> Result<(), String> {
        validation::validate_title(value, required)
    }

    pub fn validate_keywords(keywords: &params::KeywordsParam) -> Result<Vec<String>, String> {
        validation::validate_keywords(keywords)
    }

    pub fn read_file_utf8(path: &std::path::PathBuf) -> Result<String, String> {
        filesystem::read_file_utf8(path)
    }

    pub fn write_file_utf8(path: &std::path::PathBuf, content: &str) -> Result<(), String> {
        filesystem::write_file_utf8(path, content)
    }

    pub fn is_file_empty_or_whitespace(content: &str) -> bool {
        filesystem::is_file_empty_or_whitespace(content)
    }

    pub fn get_search_paths(project_root: &str) -> Vec<std::path::PathBuf> {
        filesystem::get_search_paths(project_root)
    }

    pub fn get_create_path(project_root: &str) -> std::path::PathBuf {
        filesystem::get_create_path(project_root)
    }

    pub fn list_categories(requirements_dir: &std::path::PathBuf) -> Result<Vec<String>, String> {
        helpers::list_categories(requirements_dir)
    }

    pub fn calculate_unique_prefix(name: &str, all_names: &[String]) -> String {
        helpers::calculate_unique_prefix(name, all_names)
    }

    pub fn calculate_chapter_prefix(name: &str, all_names: &[String]) -> String {
        helpers::calculate_chapter_prefix(name, all_names)
    }

    pub fn find_category_by_prefix(
        requirements_dir: &std::path::PathBuf,
        search_prefix: &str,
    ) -> Result<String, String> {
        helpers::find_category_by_prefix(requirements_dir, search_prefix)
    }

    pub fn parse_level1_heading(line: &str) -> Option<String> {
        parsing::parse_level1_heading(line)
    }

    pub fn parse_level2_heading(line: &str) -> Option<(String, String)> {
        parsing::parse_level2_heading(line)
    }

    pub fn read_chapters_streaming(
        category_path: &std::path::PathBuf,
    ) -> Result<Vec<String>, String> {
        parsing::read_chapters_streaming(category_path)
    }

    pub fn read_requirements_streaming(
        category_path: &std::path::PathBuf,
        chapter: &str,
    ) -> Result<Vec<RequirementSummary>, String> {
        parsing::read_requirements_streaming(category_path, chapter)
    }

    pub fn find_requirement_streaming(
        category_path: &std::path::PathBuf,
        category_name: &str,
        search_index: &str,
    ) -> Result<RequirementFull, String> {
        parsing::find_requirement_streaming(category_path, category_name, search_index)
    }

    pub fn parse_index(index: &str) -> Result<(String, String, String), String> {
        parsing::parse_index(index)
    }

    pub fn handle_get_instructions(params: params::GetInstructionsParams) -> String {
        handlers::handle_get_instructions(params)
    }

    pub fn handle_get_categories(params: params::GetCategoriesParams) -> String {
        handlers::handle_get_categories(params)
    }

    pub fn handle_get_chapters(params: params::GetChaptersParams) -> String {
        handlers::handle_get_chapters(params)
    }

    pub fn handle_get_requirements(params: params::GetRequirementsParams) -> String {
        handlers::handle_get_requirements(params)
    }

    pub fn handle_get_requirement(params: params::GetRequirementParams) -> String {
        handlers::handle_get_requirement(params)
    }

    pub fn handle_insert_requirement(params: params::InsertRequirementParams) -> String {
        handlers::handle_insert_requirement(params)
    }

    pub fn handle_update_requirement(params: params::UpdateRequirementParams) -> String {
        handlers::handle_update_requirement(params)
    }

    pub fn handle_get_version(params: params::GetVersionParams) -> String {
        handlers::handle_get_version(params)
    }

    pub fn handle_delete_requirement(params: params::DeleteRequirementParams) -> String {
        handlers::handle_delete_requirement(params)
    }

    pub fn handle_search_requirements(params: params::SearchRequirementsParams) -> String {
        handlers::handle_search_requirements(params)
    }
}

// ServerHandler implementation is in server.rs module
