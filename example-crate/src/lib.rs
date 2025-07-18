//! Example crate
use mpi::traits::Communicator;

/// Print the information about an MPI communicator
pub fn print_mpi_info<C: Communicator>(comm: &C) {
    println!("{} / {}", comm.rank(), comm.size());
}

/// Get a one or zero
pub fn i() -> i32 {
    if cfg!(feature = "one") { 1 } else { 0 }
}

/// Get a two or zero
pub fn j() -> i32 {
    if cfg!(feature = "two") { 2 } else { 0 }
}
