//? mpirun -n {{NPROCESSES}}

use mpi::environment::Universe;
use example::print_mpi_info;

fn main() {
    let universe: Universe = mpi::initialize().unwrap();
    let world = universe.world();

    print_mpi_info(&world);
}
