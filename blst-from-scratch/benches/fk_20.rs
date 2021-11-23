use criterion::{criterion_group, criterion_main, Criterion};
use kzg_bench::benches::fk20::{fk_single_da, fk_single_da_optimized, fk_multi_da_chunk_32, fk_multi_da_chunk_32_optimized};
use blst_from_scratch::kzg_types::{FsFFTSettings, FsKZGSettings, FsFK20SingleSettings, FsFK20MultiSettings, FsFr, FsG1, FsG2, FsPoly};
use blst_from_scratch::utils::generate_trusted_setup;

fn fk_single_da_(c: &mut Criterion) {
    fk_single_da::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings, FsFK20SingleSettings>(c, &generate_trusted_setup)
}

fn fk_single_da_optimized_(c: &mut Criterion) {
    fk_single_da_optimized::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings, FsFK20SingleSettings>(c, &generate_trusted_setup)
}

fn fk_multi_da_chunk_32_(c: &mut Criterion) {
    fk_multi_da_chunk_32::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings, FsFK20MultiSettings>(c, &generate_trusted_setup)
}

fn fk_multi_da_chunk_32_optimized_(c: &mut Criterion) {
    fk_multi_da_chunk_32_optimized::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings, FsFK20MultiSettings>(c, &generate_trusted_setup)
}


criterion_group!(benches, fk_single_da_, fk_single_da_optimized_, fk_multi_da_chunk_32_, fk_multi_da_chunk_32_optimized_);
criterion_main!(benches);