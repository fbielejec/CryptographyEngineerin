// RUN IT
// cargo bench
//

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use session_5::secret_data_branching::{fn_with_side_channel, SECRET};

fn left_path(c: &mut Criterion) {
    c.bench_function("take_left_path", |f| {
        f.iter(|| fn_with_side_channel(black_box(SECRET)))
    });
}

fn right_path(c: &mut Criterion) {
    c.bench_function("take_right_path", |f| {
        f.iter(|| fn_with_side_channel(black_box(0)))
    });
}

criterion_group!(benches, left_path, right_path);
criterion_main!(benches);
