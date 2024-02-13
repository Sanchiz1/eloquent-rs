//! # Eloquent Core
//!
//! `eloquent_core` is a library for building SQL queries in Rust.
//!

use builder::Bindings;

mod builder;
mod closures;
mod compiler;
pub mod shared;
mod traits;

pub use shared::WhereClauseBuilder;

pub struct Eloquent {
    bindings: Bindings,
}

impl Eloquent {
    /// Create a new instance of Eloquent with the given table name.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select("id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT id FROM users");
    /// ```
    pub fn table(name: &str) -> Self {
        Self {
            bindings: Bindings {
                select: vec![],
                insert: vec![],
                update: vec![],
                table: name.to_string(),
                join: vec![],
                r#where: vec![],
                where_closure: vec![],
                group_by: vec![],
                having: vec![],
                order_by: vec![],
                is_delete: false,
                limit: None,
                offset: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    String(String),
    Int(u32),
    Bool(bool),
    Null,
    Array(Vec<ArrayVariable>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayVariable {
    String(String),
    Int(u32),
}

pub enum Direction {
    Asc,
    Desc,
}
