//! Transkriptions-Subsystem fuer whisper-pill.
//!
//! Kapselt die lokale Whisper-Inferenz-Engine (whisper-rs).

pub mod accumulator;
pub mod downloader;
pub mod engine;

pub use accumulator::append_transcript;
