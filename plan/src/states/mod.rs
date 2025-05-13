//!
//!  Contains functionalities to facilitate the state transitions of the plan 
//!  while building it up with different mapping algebra [operators](operator).
//!
//!  <div style="text-align:center">
//!  <img src="https://raw.githubusercontent.com/RMLio/algemaploom-rs/refs/heads/main/plan/plan_state.png"/>
//!  <p>
//!  Figure 1. Using state transition, while building the mapping plan, limits the kind of functions that could be
//!  called for a more guided plan building.
//!  </p>
//!  </div>
//!
//!
//!  Root level contains unit structs to describe each states: 
//!
//!  1) [Init]
//!  2) [Processed]
//!  3) [Serialized]
//!  4) [Sunk]
//!
pub mod join;
pub mod processed;
pub mod serialized;
pub mod start;

#[derive(Debug, Clone)]
pub struct Init {}
#[derive(Debug, Clone)]
pub struct Processed {}
#[derive(Debug, Clone)]
pub struct Serialized {}
#[derive(Debug, Clone)]
pub struct Sunk {}
