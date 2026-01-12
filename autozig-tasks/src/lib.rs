pub mod task_pool;
pub mod executor;
pub mod iter;
pub mod task;

pub use task_pool::{TaskPool, TaskPoolBuilder, Scope};
pub use executor::{Executor, LocalExecutor, ThreadExecutor, ThreadExecutorTicker};
pub use task::Task;
pub use iter::{
    ParallelIterator, ParallelSlice, ParallelSliceMut,
    Chain, Cloned, Copied, Cycle, Filter, FilterMap,
    FlatMap, Flatten, Fuse, Inspect, Map,
};

// Conditional Send/Sync traits for WASM compatibility
pub use iter::{ConditionalSend, ConditionalSendFuture, MaybeSend, MaybeSync};

pub mod prelude {
    pub use crate::task_pool::TaskPool;
    pub use crate::task::Task;
    pub use crate::executor::Executor;
}
