use crate::{compiler::build_statement, error::EloquentError, Action, QueryBuilder};

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            table: None,
            selects: Vec::new(),
            inserts: Vec::new(),
            updates: Vec::new(),
            delete: false,
            conditions: Vec::new(),
            closures: Vec::new(),
            joins: Vec::new(),
            havings: Vec::new(),
            group_by: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            enable_checks: true,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());

        self
    }

    pub fn skip_validation(mut self) -> Self {
        self.enable_checks = false;

        self
    }

    pub fn sql(self) -> Result<String, EloquentError> {
        build_statement(&self)
    }

    pub fn pretty_sql(self) -> Result<String, EloquentError> {
        let unformatted_sql = build_statement(&self)?;

        let options = sqlformat::FormatOptions {
            indent: sqlformat::Indent::Spaces(4),
            uppercase: true,
            lines_between_queries: 2,
        };

        let sql = sqlformat::format(&unformatted_sql, &sqlformat::QueryParams::None, options);

        Ok(sql)
    }

    pub fn get_action(&self) -> Action {
        if !self.selects.is_empty() {
            Action::Select
        } else if !self.inserts.is_empty() {
            Action::Insert
        } else if !self.updates.is_empty() {
            Action::Update
        } else if self.delete {
            Action::Delete
        } else {
            Action::Select
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
