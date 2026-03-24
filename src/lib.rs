//! # spikenaut-ghost
//!
//! Bio-inspired ghost trading engine with cellular ATP energy metaphors.
//!
//! Ghost trading simulates a multi-asset portfolio without real funds,
//! using biological energy concepts to constrain position sizing:
//!
//! - **ATP** (`CELLULAR_ATP = 500 USDT`) — total available energy
//! - **Energy commitment** (8% per signal) — fraction risked per trade
//! - **Metabolic cost** (0.1% per action) — friction / spread simulation
//!
//! All trades are logged to JSONL for SNN training data and performance analysis.
//!
//! ## Usage
//!
//! ```rust
//! use spikenaut_ghost::{GhostWallet, MarketPrices, execute_buy};
//!
//! let mut wallet = GhostWallet::new();
//! let prices = MarketPrices { dnx: 0.027, sol: 90.0, ..Default::default() };
//!
//! execute_buy(&mut wallet, "DNX", prices.dnx, 1, "bull signal", None);
//! println!("Portfolio: ${:.2}", wallet.portfolio_value(&prices));
//! ```

pub mod wallet;
pub mod engine;
pub mod log;

pub use wallet::{GhostWallet, MarketPrices};
pub use engine::{execute_buy, execute_sell, CELLULAR_ATP, ENERGY_COMMITMENT, METABOLIC_COST};
pub use log::GhostTradeLog;
