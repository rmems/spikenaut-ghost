//! Ghost trade execution engine.

use crate::log::{append_ghost_log, GhostTradeLog};
use crate::wallet::GhostWallet;
use std::collections::HashMap;

/// Initial biological energy currency (USDT).
pub const CELLULAR_ATP: f32 = 500.0;

/// Fraction of available energy committed per trade signal (8%).
pub const ENERGY_COMMITMENT: f32 = 0.08;

/// Metabolic cost per action — models spread + slippage (0.1%).
pub const METABOLIC_COST: f32 = 0.001;

/// Execute a ghost buy order.
///
/// Spends `wallet.trade_fraction * wallet.balance_atp` ATP after deducting
/// the metabolic cost, then updates the weighted-average cost basis.
pub fn execute_buy(
    wallet: &mut GhostWallet,
    asset: &str,
    price: f32,
    step: u64,
    reason: &str,
    log_path: Option<&str>,
) {
    let spend_usdt = wallet.balance_atp * wallet.trade_fraction;
    if spend_usdt < 0.01 {
        return;
    }

    let fee = spend_usdt * METABOLIC_COST;
    let net_spend = spend_usdt - fee;
    let qty = net_spend / price.max(1e-9);

    wallet.apply_buy(asset, qty, net_spend);
    wallet.balance_atp -= spend_usdt;
    wallet.trade_count += 1;

    let record = GhostTradeLog {
        timestamp: chrono::Utc::now().to_rfc3339(),
        step,
        action: "buy".to_string(),
        asset: asset.to_string(),
        price_usd: price,
        quantity: qty,
        trade_value_usdt: spend_usdt,
        realized_pnl_usdt: -fee,
        balance_atp: wallet.balance_atp,
        cumulative_pnl: wallet.cumulative_pnl,
        reason: reason.to_string(),
    };

    eprintln!(
        "[ghost BUY ] step={:>5}  asset={:<6} qty={:.4} @ ${:.4}  fee=${:.4}",
        step, asset, qty, price, fee
    );

    if let Some(path) = log_path {
        append_ghost_log(&record, path);
    }
}

/// Execute a ghost sell order.
///
/// Sells `wallet.trade_fraction * balance[asset]` units, deducting metabolic cost.
pub fn execute_sell(
    wallet: &mut GhostWallet,
    asset: &str,
    price: f32,
    step: u64,
    reason: &str,
    log_path: Option<&str>,
) {
    let qty = wallet.balance(asset) * wallet.trade_fraction;
    if qty < 1e-9 {
        return;
    }

    let entry_price = wallet.entry_price(asset);
    let proceeds = qty * price;
    let fee = proceeds * METABOLIC_COST;
    let net_proceeds = proceeds - fee;
    let pnl = (price - entry_price) * qty - fee;

    wallet.apply_sell(asset, qty);
    wallet.balance_atp += net_proceeds;
    wallet.cumulative_pnl += pnl;
    wallet.trade_count += 1;
    wallet.record_pnl_and_update_kelly(pnl);

    let record = GhostTradeLog {
        timestamp: chrono::Utc::now().to_rfc3339(),
        step,
        action: "sell".to_string(),
        asset: asset.to_string(),
        price_usd: price,
        quantity: qty,
        trade_value_usdt: proceeds,
        realized_pnl_usdt: pnl,
        balance_atp: wallet.balance_atp,
        cumulative_pnl: wallet.cumulative_pnl,
        reason: reason.to_string(),
    };

    eprintln!(
        "[ghost SELL] step={:>5}  asset={:<6} qty={:.4} @ ${:.4}  PnL={:+.4}",
        step, asset, qty, price, pnl
    );

    if let Some(path) = log_path {
        append_ghost_log(&record, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_buy_reduces_usdt() {
        let mut wallet = GhostWallet::new();
        let before = wallet.balance_atp;
        execute_buy(&mut wallet, "DNX", 0.03, 1, "test", None);
        assert!(wallet.balance_atp < before);
    }

    #[test]
    fn test_sell_increases_usdt() {
        let mut wallet = GhostWallet::new();
        wallet.balances.insert("DNX".to_string(), 1_000.0);
        wallet.entry_prices.insert("DNX".to_string(), 0.02);
        let before = wallet.balance_atp;
        execute_sell(&mut wallet, "DNX", 0.05, 1, "test", None);
        assert!(wallet.balance_atp > before, "sell at profit should increase ATP");
    }

    #[test]
    fn test_trade_count_increments() {
        let mut wallet = GhostWallet::new();
        execute_buy(&mut wallet, "SOL", 90.0, 1, "test", None);
        execute_sell(&mut wallet, "SOL", 95.0, 2, "test", None);
        assert_eq!(wallet.trade_count, 2);
    }

    #[test]
    fn test_portfolio_value() {
        let wallet = GhostWallet::new();
        let prices = HashMap::from([
            ("DNX".to_string(), 0.0266),
            ("SOL".to_string(), 86.0),
            ("RENDER".to_string(), 1.52),
            ("ASI".to_string(), 0.0616),
            ("NEAR".to_string(), 1.31),
            ("BTC".to_string(), 70_000.0),
            ("PEPE".to_string(), 0.000_003_35),
        ]);
        let value = wallet.portfolio_value(&prices);
        assert_eq!(value, wallet.balance_atp);
    }
}