//? mpirun -n {{NPROCESSES}}

use example::print_mpi_info;
use mpi::environment::Universe;

fn main() {
    let universe: Universe = mpi::initialize().unwrap();
    let world = universe.world();

    print_mpi_info(&world);
}
