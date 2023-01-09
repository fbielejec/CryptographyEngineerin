//
// cargo bench
//

#![allow(unused_imports)]
use std::{thread, time};

use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use rsa_benchmark::RSA;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rsa = RSA::default();

    c.bench_function("encryp+decrypt", |f| {
        f.iter(|| {
            let message = String::from("Hello World");
            let cyphertext = rsa.encrypt(message.clone()).unwrap();
            let decrypted_message = rsa.decrypt(cyphertext).unwrap();

            assert_eq!(message, decrypted_message);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
