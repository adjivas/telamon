//! Code generation and candidate evaluation for specific targets.
pub mod cuda;
#[cfg(feature = "mppa")]
pub mod mppa;
pub mod x86;

mod argument;
mod context;

pub use self::argument::{read_array, write_array, ArrayArgument, ScalarArgument};
pub use self::context::{ArgMap, AsyncCallback, AsyncEvaluator, Context, EvalMode};

use codegen::Function;
use ir;
use model::{self, HwPressure, Nesting};
use search_space::{DimKind, SearchSpace};
use std::io::Write;
use utils::*;

/// Holds the specifications of a target.
pub trait Device: Sync {
    /// Prints the code corresponding to a device `Function`.
    fn print(&self, function: &Function, out: &mut Write);
    /// Indicates if a `Type` can be implemented on the device.
    fn check_type(&self, t: ir::Type) -> Result<(), ir::TypeError>;
    /// Returns the maximal number of block dimensions.
    fn max_block_dims(&self) -> u32;
    /// The maximal size inner block dimensions can have.
    fn max_inner_block_size(&self) -> u32;
    /// Returns the maximal number of threads.
    fn max_threads(&self) -> u32;
    /// Returns the maximal unrolling factor.
    fn max_unrolling(&self) -> u32;
    /// Indicates the valid vectorization factors for the given operator, on the given
    /// dimension.
    fn vectorization_factors(&self, dim: &ir::Dimension, op: &ir::Operator) -> &[u32];
    /// Returns the amount of shared memory available for each thread block.
    fn shared_mem(&self) -> u32;
    /// Indicates if the device supports non-coherent memory accesses.
    fn supports_nc_access(&self) -> bool;
    /// Indicates if the device supports L1 for global memory accesses.
    fn supports_l1_access(&self) -> bool;
    /// Indicates if the device supports L2 for global memory accesses.
    fn supports_l2_access(&self) -> bool;
    /// Returns the name of the device.
    fn name(&self) -> &str;

    /// Returns the pressure cause by a `Statement`. For a dimension, returns the pressure
    /// for the full loop execution.
    fn hw_pressure(
        &self,
        space: &SearchSpace,
        dim_sizes: &HashMap<ir::DimId, model::size::Range>,
        nesting: &HashMap<ir::StmtId, Nesting>,
        bb: &ir::Statement,
        ctx: &Context,
    ) -> HwPressure;
    /// Returns the pressure produced by a single iteration of a loop and the latency
    /// overhead of iterations.
    fn loop_iter_pressure(&self, kind: DimKind) -> (HwPressure, HwPressure);
    /// Returns the processing rates of a single thread, in units/ns
    fn thread_rates(&self) -> HwPressure;
    /// Returns the processing rates of a single block, in units/ns.
    fn block_rates(&self) -> HwPressure;
    /// Returns the processing rates of the whole accelerator un units/ns.
    fn total_rates(&self) -> HwPressure;
    /// Returns the names of potential bottlenecks.
    fn bottlenecks(&self) -> &[&'static str];
    /// Returns the number of blocks that can be executed in parallel on the device.
    fn block_parallelism(&self, space: &SearchSpace) -> u32;
    /// Returns the pressure caused by an additive induction variable level.
    fn additive_indvar_pressure(&self, t: &ir::Type) -> HwPressure;
    /// Returns the pressure caused by a multiplicative induction variable level.
    fn multiplicative_indvar_pressure(&self, t: &ir::Type) -> HwPressure;
    /// Adds the overhead (per instance) due to partial wraps and predicated dimensions to
    /// the pressure. If the instruction is not predicated, `predicated_dims_size` should
    /// be `1`.
    fn add_block_overhead(
        &self,
        max_active_threads: model::size::FactorRange,
        max_threads: model::size::FactorRange,
        predication_factor: model::size::Range,
        pressure: &mut HwPressure,
    );

    /// Lowers a type using the memory space information. Returns `None` if some
    /// information is not yet specified.
    fn lower_type(&self, t: ir::Type, space: &SearchSpace) -> Option<ir::Type>;

    /// Builds and outputs a constrained IR instance.
    fn gen_code(&self, implementation: &SearchSpace, out: &mut Write) {
        let code = Function::build(implementation);
        self.print(&code, out);
    }
}
