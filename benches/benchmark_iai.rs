use iai::black_box;
use sunscreen_iai_bench::{run_fhe, run_non_fhe, setup_fhe, setup_non_fhe};

fn non_fhe_dot_product() {
    let setup = setup_non_fhe();
    run_non_fhe(black_box(setup));
}

fn fhe_dot_product() {
    let setup = setup_fhe();
    run_fhe(black_box(setup));
}

iai::main!(non_fhe_dot_product, fhe_dot_product);
