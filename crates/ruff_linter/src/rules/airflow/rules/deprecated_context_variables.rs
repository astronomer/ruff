use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::{Expr, ExprName};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for usage of deprecated Airflow context variables.
///
/// ## Why is this bad?
/// These context variables have been removed in Airflow 3.0. Using them will
/// cause runtime errors.
///
/// ## Example
/// ```python
/// execution_date = context['execution_date']
/// ```
///
/// This will raise an error in Airflow 3.0. Remove the use of these variables or replace them with alternatives.
#[derive(ViolationMetadata)]
pub(crate) struct DeprecatedContextVariable {
    variable: String,
}

impl Violation for DeprecatedContextVariable {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DeprecatedContextVariable { variable } = self;
        format!("`{variable}` is a deprecated Airflow context variable and is removed in Airflow 3.0")
    }
}

/// AIR303
pub(crate) fn deprecated_context_variable(checker: &mut Checker, expr: &Expr) {
    // List of deprecated context variables
    let deprecated_variables = [
        "execution_date",
        "next_ds",
        "next_ds_nodash",
        "next_execution_date",
        "prev_ds",
        "prev_ds_nodash",
        "prev_execution_date",
        "prev_execution_date_success",
        "tomorrow_ds",
        "yesterday_ds",
        "yesterday_ds_nodash",
    ];

    // Check if the expression is a name and matches any deprecated variable
    if let Expr::Name(ExprName { id, range }) = expr {
        if deprecated_variables.contains(&id.as_str()) {
            checker.diagnostics.push(Diagnostic::new(
                DeprecatedContextVariable {
                    variable: id.to_string(),
                },
                *range,
            ));
        }
    }
}
