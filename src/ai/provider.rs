use async_trait::async_trait;

use super::error::AIError;

/// Trait that defines the interface for AI providers
/// This allows easy swapping between different AI services (Gemini, OpenAI, Claude, etc.)
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Send a prompt to the AI provider and get a response
    async fn generate(&self, prompt: &str) -> Result<String, AIError>;

    /// Get the name of the provider (for logging/debugging)
    fn name(&self) -> &str;
}
