//! Example crate
use mpi::traits::Communicator;

/// Print the information about an MPI communicator
pub fn print_mpi_info<C: Communicator>(comm: &C) {
    println!("{} / {}", comm.rank(), comm.size());
}
