<p align="center">
  <img src="docs/logo.png" width="220" alt="Spikenaut">
</p>

<h1 align="center">spikenaut-ghost</h1>
<p align="center">Bio-inspired ghost trading engine with ATP cellular energy metaphors</p>

<p align="center">
  <a href="https://crates.io/crates/spikenaut-ghost"><img src="https://img.shields.io/crates/v/spikenaut-ghost" alt="crates.io"></a>
  <a href="https://docs.rs/spikenaut-ghost"><img src="https://docs.rs/spikenaut-ghost/badge.svg" alt="docs.rs"></a>
  <img src="https://img.shields.io/badge/license-GPL--3.0-orange" alt="GPL-3.0">
</p>

---

Simulates a multi-asset portfolio without real funds, using biological energy
concepts to constrain position sizing. Each trade signal consumes a fraction of
available ATP, with a metabolic cost modelling spread/slippage. All decisions are
logged to JSONL for SNN training data and post-hoc analysis.

## Features

- `GhostWallet` — 7-asset portfolio (BTC, ETH, SOL, DNX, QUAI, QUBIC, USDT) with weighted-average cost basis
- `execute_buy` / `execute_sell` — ATP-gated order execution with metabolic cost
- `CELLULAR_ATP = 500.0` — initial energy budget (USDT equivalent)
- `ENERGY_COMMITMENT = 0.08` — 8% of available energy per signal
- `METABOLIC_COST = 0.001` — 0.1% friction per action
- `GhostTradeLog` — JSONL audit trail with timestamp, asset, side, price, units, reason
- Kelly fraction auto-update after 10+ trades based on realized win rate

## Installation

```toml
spikenaut-ghost = "0.1"
```

## Quick Start

```rust
use spikenaut_ghost::{GhostWallet, MarketPrices, execute_buy, execute_sell};

let mut wallet = GhostWallet::new();
let prices = MarketPrices { dnx: 0.027, sol: 90.0, ..Default::default() };

// SNN fires a BUY signal for DNX
execute_buy(&mut wallet, "DNX", prices.dnx, 1, "bull signal: confidence=0.92", None);

println!("Portfolio: ${:.2}", wallet.portfolio_value(&prices));
println!("DNX units: {:.2}", wallet.positions["DNX"]);
```

## With JSONL Audit Log

```rust
execute_buy(&mut wallet, "BTC", 65_000.0, 1, "snn_fire", Some("trades.jsonl"));
// Appends: {"ts":"2024-...","asset":"BTC","side":"BUY","price":65000.0,...}
```

## Energy Model

```
available_energy = wallet.usdt_balance
committed        = available_energy × ENERGY_COMMITMENT   (8%)
units            = committed / price
cost             = committed × METABOLIC_COST             (0.1% friction)
wallet.usdt     -= committed + cost
```

Inspired by ATP as cellular energy currency (Alberts et al. 2002) and half-Kelly
position sizing (Kelly 1956; Thorp 1969).

## Part of the Spikenaut Ecosystem

| Library | Purpose |
|---------|---------|
| [SpikenautKelly.jl](https://github.com/rmems/SpikenautKelly.jl) | Kelly position sizing in Julia |
| [SpikenautExecution.jl](https://github.com/rmems/SpikenautExecution.jl) | Live dYdX v4 execution pipeline |
| [neuromod](https://github.com/rmems/neuromod) | LIF/Izhikevich SNN generating trade signals |

## Provenance

Extracted from Eagle-Lander, the author's own private neuromorphic GPU supervisor
repository (closed-source). Ghost-traded Dynex/Quai/Qubic/BTC portfolios in
production alongside the live SNN supervisor.

## License

GPL-3.0-or-later
