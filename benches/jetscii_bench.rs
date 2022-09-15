use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use jetscii::{ascii_chars, Substring};
use std::time::Duration;

fn prep_haystack(len: usize, needle: &str) -> String {
    let mut haystack = "a".repeat(len - needle.len());
    haystack.push_str(needle);
    haystack
}

fn space_ascii_chars(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    let space = &ascii_chars!(' ');
    b.iter(|| black_box(space.find(hs)));
}

fn xml_delim_3_ascii_chars(b: &mut Bencher, haystack_len: usize) {
    let xml_delim_3 = &ascii_chars!('<', '>', '&');
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(xml_delim_3.find(hs)));
}

fn xml_delim_5_ascii_chars(b: &mut Bencher, haystack_len: usize) {
    let xml_delim_5 = &ascii_chars!('<', '>', '&', '\'', '"');
    let hs: &str = &prep_haystack(haystack_len, "\"");
    b.iter(|| black_box(xml_delim_5.find(hs)));
}

fn substring_with_created_searcher(b: &mut Bencher, haystack_len: usize) {
    let needle = "xyzzy";
    let xyzzy = &Substring::new(needle);
    let hs: &str = &prep_haystack(haystack_len, needle);
    b.iter(|| black_box(xyzzy.find(hs)));
}

macro_rules! benchmark {
    ($criterion_obj:ident, $func_name:ident) => {
        $criterion_obj.bench_function(stringify!($func_name), |b| $func_name(b));
    };
    ($criterion_obj:ident, $func_name:ident, $haystack_len:expr) => {
        $criterion_obj.bench_function(concat!(stringify!($func_name), "_", $haystack_len), |b| {
            $func_name(b, $haystack_len)
        });
    };
}

fn do_bench(c: &mut Criterion) {
    benchmark!(c, space_ascii_chars, 64);
    benchmark!(c, space_ascii_chars, 128);
    benchmark!(c, space_ascii_chars, 256);
    benchmark!(c, space_ascii_chars, 1024);
    benchmark!(c, space_ascii_chars, 0x500000);
    benchmark!(c, xml_delim_3_ascii_chars, 64);
    benchmark!(c, xml_delim_3_ascii_chars, 128);
    benchmark!(c, xml_delim_3_ascii_chars, 256);
    benchmark!(c, xml_delim_3_ascii_chars, 512);
    benchmark!(c, xml_delim_3_ascii_chars, 1024);
    benchmark!(c, xml_delim_3_ascii_chars, 0x500000);
    benchmark!(c, xml_delim_5_ascii_chars, 64);
    benchmark!(c, xml_delim_5_ascii_chars, 128);
    benchmark!(c, xml_delim_5_ascii_chars, 256);
    benchmark!(c, xml_delim_5_ascii_chars, 512);
    benchmark!(c, xml_delim_5_ascii_chars, 1024);
    benchmark!(c, xml_delim_5_ascii_chars, 0x500000);
    benchmark!(c, substring_with_created_searcher, 64);
    benchmark!(c, substring_with_created_searcher, 128);
    benchmark!(c, substring_with_created_searcher, 256);
    benchmark!(c, substring_with_created_searcher, 512);
    benchmark!(c, substring_with_created_searcher, 1024);
    benchmark!(c, substring_with_created_searcher, 0x500000);
}

fn default_config() -> Criterion {
    Criterion::default()
        .significance_level(0.05)
        .sample_size(500)
        .measurement_time(Duration::from_secs(20))
}

criterion_group! {
    name = bench_jetscii;
    config = default_config();
    targets = do_bench
}

criterion_main!(bench_jetscii);
