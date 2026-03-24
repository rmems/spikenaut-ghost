//! Ghost wallet — multi-asset virtual portfolio with Kelly position sizing.

use crate::engine::{CELLULAR_ATP, ENERGY_COMMITMENT};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Current market prices for all supported assets (USD).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketPrices {
    pub dnx: f32,
    pub sol: f32,
    pub render: f32,
    pub asi: f32,
    pub near: f32,
    pub btc: f32,
    pub pepe: f32,
}

impl MarketPrices {
    pub fn get(&self, asset: &str) -> f32 {
        match asset {
            "DNX" => self.dnx,
            "SOL" => self.sol,
            "RENDER" => self.render,
            "ASI" => self.asi,
            "NEAR" => self.near,
            "BTC" => self.btc,
            "PEPE" => self.pepe,
            _ => 0.0,
        }
    }
}

/// Virtual ghost-trading wallet with biological ATP energy model.
pub struct GhostWallet {
    // ── USDT base (cellular ATP) ──────────────────────────────────────────
    pub balance_usdt: f32,

    // ── Token positions ───────────────────────────────────────────────────
    pub balance_dnx: f32,
    pub balance_sol: f32,
    pub balance_render: f32,
    pub balance_asi: f32,
    pub balance_near: f32,
    pub balance_btc: f32,
    pub balance_pepe: f32,

    // ── Weighted-average cost basis ───────────────────────────────────────
    pub entry_price_dnx: f32,
    pub entry_price_sol: f32,
    pub entry_price_render: f32,
    pub entry_price_asi: f32,
    pub entry_price_near: f32,
    pub entry_price_btc: f32,
    pub entry_price_pepe: f32,

    // ── Performance tracking ──────────────────────────────────────────────
    pub cumulative_pnl: f32,
    pub trade_count: u64,

    // ── Kelly criterion state ─────────────────────────────────────────────
    pub win_count: u64,
    pub loss_count: u64,
    pub total_win: f32,
    pub total_loss: f32,
    /// Adaptive trade fraction, initialized to `ENERGY_COMMITMENT`.
    pub trade_fraction: f32,
    pub price_history: VecDeque<f32>,
}

impl GhostWallet {
    /// Create a new ghost wallet with diversified initial positions.
    pub fn new() -> Self {
        Self {
            balance_usdt: CELLULAR_ATP,
            balance_dnx: 50.0,
            balance_sol: 2.5,
            balance_render: 50.0,
            balance_asi: 500.0,
            balance_near: 100.0,
            balance_btc: 0.002,
            balance_pepe: 1_000_000.0,
            entry_price_dnx: 0.0266,
            entry_price_sol: 86.0,
            entry_price_render: 1.52,
            entry_price_asi: 0.0616,
            entry_price_near: 1.31,
            entry_price_btc: 70_000.0,
            entry_price_pepe: 0.000_003_35,
            cumulative_pnl: 0.0,
            trade_count: 0,
            win_count: 0,
            loss_count: 0,
            total_win: 0.0,
            total_loss: 0.0,
            trade_fraction: ENERGY_COMMITMENT,
            price_history: VecDeque::with_capacity(50),
        }
    }

    /// Total portfolio value in USDT at current prices.
    pub fn portfolio_value(&self, prices: &MarketPrices) -> f32 {
        self.balance_usdt
            + self.balance_dnx * prices.dnx
            + self.balance_sol * prices.sol
            + self.balance_render * prices.render
            + self.balance_asi * prices.asi
            + self.balance_near * prices.near
            + self.balance_btc * prices.btc
            + self.balance_pepe * prices.pepe
    }

    /// Get token balance for a named asset.
    pub fn balance(&self, asset: &str) -> f32 {
        match asset {
            "DNX" => self.balance_dnx,
            "SOL" => self.balance_sol,
            "RENDER" => self.balance_render,
            "ASI" => self.balance_asi,
            "NEAR" => self.balance_near,
            "BTC" => self.balance_btc,
            "PEPE" => self.balance_pepe,
            _ => 0.0,
        }
    }

    /// Get cost basis (entry price) for a named asset.
    pub fn entry_price(&self, asset: &str) -> f32 {
        match asset {
            "DNX" => self.entry_price_dnx,
            "SOL" => self.entry_price_sol,
            "RENDER" => self.entry_price_render,
            "ASI" => self.entry_price_asi,
            "NEAR" => self.entry_price_near,
            "BTC" => self.entry_price_btc,
            "PEPE" => self.entry_price_pepe,
            _ => 0.0,
        }
    }

    /// Update balance and cost basis after a buy.
    pub(crate) fn apply_buy(&mut self, asset: &str, qty: f32, net_spend: f32) {
        match asset {
            "DNX" => {
                let prev_cost = self.entry_price_dnx * self.balance_dnx;
                self.balance_dnx += qty;
                if self.balance_dnx > 1e-9 {
                    self.entry_price_dnx = (prev_cost + net_spend) / self.balance_dnx;
                }
            }
            "SOL" => {
                let prev_cost = self.entry_price_sol * self.balance_sol;
                self.balance_sol += qty;
                if self.balance_sol > 1e-9 {
                    self.entry_price_sol = (prev_cost + net_spend) / self.balance_sol;
                }
            }
            "RENDER" => {
                let prev_cost = self.entry_price_render * self.balance_render;
                self.balance_render += qty;
                if self.balance_render > 1e-9 {
                    self.entry_price_render = (prev_cost + net_spend) / self.balance_render;
                }
            }
            "ASI" => {
                let prev_cost = self.entry_price_asi * self.balance_asi;
                self.balance_asi += qty;
                if self.balance_asi > 1e-9 {
                    self.entry_price_asi = (prev_cost + net_spend) / self.balance_asi;
                }
            }
            "NEAR" => {
                let prev_cost = self.entry_price_near * self.balance_near;
                self.balance_near += qty;
                if self.balance_near > 1e-9 {
                    self.entry_price_near = (prev_cost + net_spend) / self.balance_near;
                }
            }
            "BTC" => {
                let prev_cost = self.entry_price_btc * self.balance_btc;
                self.balance_btc += qty;
                if self.balance_btc > 1e-9 {
                    self.entry_price_btc = (prev_cost + net_spend) / self.balance_btc;
                }
            }
            "PEPE" => {
                let prev_cost = self.entry_price_pepe * self.balance_pepe;
                self.balance_pepe += qty;
                if self.balance_pepe > 1e-9 {
                    self.entry_price_pepe = (prev_cost + net_spend) / self.balance_pepe;
                }
            }
            _ => {}
        }
    }

    /// Reduce balance after a sell.
    pub(crate) fn apply_sell(&mut self, asset: &str, qty: f32) {
        match asset {
            "DNX" => self.balance_dnx -= qty,
            "SOL" => self.balance_sol -= qty,
            "RENDER" => self.balance_render -= qty,
            "ASI" => self.balance_asi -= qty,
            "NEAR" => self.balance_near -= qty,
            "BTC" => self.balance_btc -= qty,
            "PEPE" => self.balance_pepe -= qty,
            _ => {}
        }
    }

    /// Record closed trade PnL and update Kelly fraction.
    pub fn record_pnl_and_update_kelly(&mut self, pnl: f32) {
        if pnl > 0.0 {
            self.win_count += 1;
            self.total_win += pnl;
        } else if pnl < 0.0 {
            self.loss_count += 1;
            self.total_loss += pnl.abs();
        }

        let closed = self.win_count + self.loss_count;
        if closed < 10 {
            return;
        } // need minimum sample

        let win_rate = self.win_count as f64 / closed as f64;
        let avg_win = if self.win_count > 0 {
            self.total_win as f64 / self.win_count as f64
        } else {
            0.0
        };
        let avg_loss = if self.loss_count > 0 {
            self.total_loss as f64 / self.loss_count as f64
        } else {
            0.0
        };

        if avg_win < 1e-6 || avg_loss < 1e-6 {
            return;
        }

        let b = avg_win / avg_loss;
        let q = 1.0 - win_rate;
        let full_kelly = (win_rate * b - q) / b;
        let half_kelly = (full_kelly * 0.5).clamp(0.02, 0.20) as f32;
        self.trade_fraction = half_kelly;
    }

    /// Win rate from closed trades (returns `None` if no trades).
    pub fn win_rate(&self) -> Option<f64> {
        let closed = self.win_count + self.loss_count;
        if closed == 0 {
            None
        } else {
            Some(self.win_count as f64 / closed as f64)
        }
    }
}

impl Default for GhostWallet {
    fn default() -> Self {
        Self::new()
    }
}
