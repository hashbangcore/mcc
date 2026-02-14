//! Task modules that power CLI commands.
/// Interactive chat task.
pub mod chat;
/// Commit message generation task.
pub mod commit;
/// Shared attachment helpers for tasks.
pub mod attach;
/// Single prompt pipeline task.
pub mod pipeline;
/// Shared helpers for task output.
pub mod render;
