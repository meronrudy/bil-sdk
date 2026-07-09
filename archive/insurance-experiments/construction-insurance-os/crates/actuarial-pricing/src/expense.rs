//! Expense and profit loading.
//! Premium = (Loss + LAE) / (1 - V - Q)

use serde::{Deserialize, Serialize};
use actuarial_core::ActuarialError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpenseAssumptions {
    pub commission_rate: f64,
    pub premium_tax_rate: f64,
    pub general_expense_ratio: f64,
    pub allocated_lae_ratio: f64,
    pub unallocated_lae_ratio: f64,
    pub target_profit_margin: f64,
    pub risk_margin: f64,
    pub catastrophe_load: f64,
}

impl Default for ExpenseAssumptions {
    fn default() -> Self {
        Self {
            commission_rate: 0.10,
            premium_tax_rate: 0.03,
            general_expense_ratio: 0.06,
            allocated_lae_ratio: 0.08,
            unallocated_lae_ratio: 0.04,
            target_profit_margin: 0.05,
            risk_margin: 0.03,
            catastrophe_load: 0.02,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpenseLoadedPremium {
    pub pure_premium: f64,
    pub lae_provision: f64,
    pub lae_loaded: f64,
    pub variable_expense_ratio: f64,
    pub profit_and_contingency: f64,
    pub permissible_loss_ratio: f64,
    pub gross_premium: f64,
    pub implied_loss_ratio: f64,
}

pub fn apply_expense_loading(
    pure_premium: f64,
    assumptions: &ExpenseAssumptions,
) -> Result<ExpenseLoadedPremium, ActuarialError> {
    let lae_loaded = pure_premium * (1.0 + assumptions.allocated_lae_ratio + assumptions.unallocated_lae_ratio);
    let v = assumptions.commission_rate + assumptions.premium_tax_rate + assumptions.general_expense_ratio;
    let q = assumptions.target_profit_margin + assumptions.risk_margin + assumptions.catastrophe_load;
    let denom = 1.0 - v - q;

    if denom <= 0.0 {
        return Err(ActuarialError::InvalidExpenseLoading(
            format!("loads exceed 100%: V={v:.2%}, Q={q:.2%}")
        ));
    }

    let gross = lae_loaded / denom;

    Ok(ExpenseLoadedPremium {
        pure_premium,
        lae_provision: lae_loaded - pure_premium,
        lae_loaded,
        variable_expense_ratio: v,
        profit_and_contingency: q,
        permissible_loss_ratio: denom,
        gross_premium: gross,
        implied_loss_ratio: pure_premium / gross,
    })
}
