#[macro_use]
extern crate criterion;
use criterion::Criterion;

use libpaillier::{
    unknown_order::BigNumber,
    *
};

fn encrypt_ciphertext(c: &mut Criterion) {
    let res = DecryptionKey::random();
    let sk = res.unwrap();
    let pk = EncryptionKey::from(&sk);

    let m1 = BigNumber::random(&BigNumber::from(10000));

    let label = format!("Single Paillier Encryption (1024-bit primes)");
    c.bench_function(&label, move |b| {


        b.iter(|| {
            pk.encrypt(m1.to_bytes(), None);
        })
    });
}

fn decrypt_ciphertext(c: &mut Criterion) {
    let res = DecryptionKey::random();
    let sk = res.unwrap();
    let pk = EncryptionKey::from(&sk);

    let m1 = BigNumber::random(&BigNumber::from(10000));
    let res = pk.encrypt(m1.to_bytes(), None).unwrap();
    let (ctxt, _) = res;

    let label = format!("Single Paillier Decryption (1024-bit primes");
    c.bench_function(&label, move |b| {


        b.iter(|| {
            sk.decrypt(&ctxt);
        })
    });
}

criterion_group!(
    benches,
    encrypt_ciphertext,
    decrypt_ciphertext,
);
criterion_main!(benches);
