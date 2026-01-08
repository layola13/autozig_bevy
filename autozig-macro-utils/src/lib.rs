//! Macro utility functions for AutoZig Bevy implementation.
//! 
//! This crate provides utilities for procedural macros, including:
//! - Attribute parsing
//! - Cargo manifest handling
//! - Label trait derivation
//! - Symbol types for efficient string comparison
//! - Struct shape validation
//! - Field member access
//! - Custom parsers
//! - Result aggregation
//! - Fully qualified standard library types

mod attrs;
mod bevy_manifest;
mod fq_std;
mod label;
mod member;
mod parser;
mod result_sifter;
mod shape;
mod symbol;

pub use attrs::{get_lit_bool, get_lit_str};
pub use bevy_manifest::{BevyManifest, AUTOZIG_BEVY};
pub use fq_std::{FQAny, FQBox, FQClone, FQDefault, FQOption, FQResult, FQSend, FQSync};
pub use label::define_label;
pub use member::as_member;
pub use parser::terminated_parser;
pub use result_sifter::ResultSifter;
pub use shape::{get_struct_fields, require_named_fields, Fields, FieldsNamed, FieldsUnnamed};
pub use symbol::Symbol;

// Re-export commonly used types from dependencies
pub use proc_macro2::{Ident, Span, TokenStream};
pub use quote::{quote, ToTokens};
pub use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Attribute, Data, DeriveInput, Error, Expr, Field, GenericParam, Generics, Lit,
    LitBool, LitStr, Meta, Path, Result, Type, TypePath,
};