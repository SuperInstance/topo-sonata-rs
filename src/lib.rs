//! # topo-sonata-rs
//!
//! Topological analysis of musical sonata form.
//!
//! Provides sonata form structure analysis, topological transitions between
//! sections, motivic analysis using persistent homology, harmonic tension
//! landscapes, and recapitulation similarity measurement using Wasserstein distance.

pub mod form;
pub mod motif;
pub mod recapitulation;
pub mod tension;
pub mod transition;
