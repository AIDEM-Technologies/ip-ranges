use criterion::{criterion_group, criterion_main, Criterion};
use cidr_utils::cidr::Ipv4Cidr;
use itertools::Itertools;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let s = include_str!("../datacenter-cidr.txt");
    let cidr = s
        .lines()
        .map(|line| {
            let index = line.find(':').expect("Infallible");
            line[index + 1..].to_string()
        })
        .map(|s| Ipv4Cidr::from_str(&s).expect("Infallible"))
        .collect_vec();
    let range_collection = ip_ranges::IpRanges::new_from_cidr(cidr);
    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(10000);
    group.bench_function("fib 20", |b| b.iter(|| range_collection.contains_ip(random_ipv4(&mut rng))));
    group.finish();
}

fn random_ipv4(rng: &mut rand::rngs::ThreadRng) -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::new(rng.gen(),rng.gen(),rng.gen(),rng.gen())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
