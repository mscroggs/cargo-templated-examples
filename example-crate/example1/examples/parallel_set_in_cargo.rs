use example1::print_mpi_info;
use mpi::{environment::Universe, topology::Communicator};

fn main() {
    let universe: Universe = mpi::initialize().unwrap();
    let world = universe.world();

    print_mpi_info(&world);
    assert!(world.size() > 1);
}
