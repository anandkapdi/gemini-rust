//! # gemini-rust
//!
//! A Rust client library for Google's Gemini 2.0 API.

mod client;
mod content_builder;
mod embed_builder;
mod error;
mod models;
mod tools;

pub use client::Gemini;
pub use error::Error;
pub use models::{
    Candidate, CitationMetadata,  UsageMetadata, Content, FunctionCallingMode, GenerationConfig,
    GenerationResponse, Message, Part, Role, SafetyRating, TaskType
};
pub use tools::{FunctionCall, FunctionDeclaration, FunctionParameters, PropertyDetails, Tool};

/// Result type for this crate
pub type Result<T> = std::result::Result<T, Error>;
