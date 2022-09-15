use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use std::time::Duration;

fn prep_haystack(len: usize, needle: &str) -> String {
    let mut haystack = "a".repeat(len - needle.len());
    haystack.push_str(needle);
    haystack
}

fn space_stdlib_find_string(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(hs.find(" ")));
}

fn space_stdlib_find_char(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(hs.find(' ')));
}

fn space_stdlib_find_char_set(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(hs.find(&[' '][..])));
}

fn space_stdlib_find_closure(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(hs.find(|c| c == ' ')));
}

fn space_stdlib_iterator_position(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, " ");
    b.iter(|| black_box(hs.as_bytes().iter().position(|&v| v == b' ')));
}

fn xml_delim_3_stdlib_find_char_set(b: &mut Bencher, haystack_len: usize) {
    const PAT: &[char] = &['<', '>', '&'];
    let hs: &str = &prep_haystack(haystack_len, "&");
    b.iter(|| black_box(hs.find(PAT)));
}

fn xml_delim_3_stdlib_find_char_closure(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, "&");
    b.iter(|| hs.find(|c| c == '<' || c == '>' || c == '&'));
}

fn xml_delim_3_stdlib_iterator_position(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, "&");
    b.iter(|| {
        black_box(
            hs.as_bytes()
                .iter()
                .position(|&c| c == b'<' || c == b'>' || c == b'&'),
        )
    });
}

fn xml_delim_5_stdlib_find_char_set(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, "\"");
    b.iter(|| black_box(hs.find(&['<', '>', '&', '\'', '"'][..])));
}

fn xml_delim_5_stdlib_find_char_closure(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, "\"");
    b.iter(|| black_box(hs.find(|c| c == '<' || c == '>' || c == '&' || c == '\'' || c == '"')));
}

fn xml_delim_5_stdlib_iterator_position(b: &mut Bencher, haystack_len: usize) {
    let hs: &str = &prep_haystack(haystack_len, "\"");
    b.iter(|| {
        black_box(
            hs.as_bytes()
                .iter()
                .position(|&c| c == b'<' || c == b'>' || c == b'&' || c == b'\'' || c == b'"'),
        )
    });
}

fn substring_stdlib_find(b: &mut Bencher, haystack_len: usize) {
    let needle = "xyzzy";
    let hs: &str = &prep_haystack(haystack_len, needle);
    b.iter(|| black_box(hs.find(needle)));
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
    benchmark!(c, space_stdlib_find_string, 64);
    benchmark!(c, space_stdlib_find_string, 128);
    benchmark!(c, space_stdlib_find_string, 256);
    benchmark!(c, space_stdlib_find_string, 1024);
    benchmark!(c, space_stdlib_find_string, 0x500000);
    benchmark!(c, space_stdlib_find_char, 64);
    benchmark!(c, space_stdlib_find_char, 128);
    benchmark!(c, space_stdlib_find_char, 256);
    benchmark!(c, space_stdlib_find_char, 1024);
    benchmark!(c, space_stdlib_find_char, 0x500000);
    benchmark!(c, space_stdlib_find_char_set, 64);
    benchmark!(c, space_stdlib_find_char_set, 128);
    benchmark!(c, space_stdlib_find_char_set, 256);
    benchmark!(c, space_stdlib_find_char_set, 512);
    benchmark!(c, space_stdlib_find_char_set, 1024);
    benchmark!(c, space_stdlib_find_char_set, 0x500000);
    benchmark!(c, space_stdlib_find_closure, 64);
    benchmark!(c, space_stdlib_find_closure, 128);
    benchmark!(c, space_stdlib_find_closure, 256);
    benchmark!(c, space_stdlib_find_closure, 512);
    benchmark!(c, space_stdlib_find_closure, 1024);
    benchmark!(c, space_stdlib_find_closure, 0x500000);
    benchmark!(c, space_stdlib_iterator_position, 64);
    benchmark!(c, space_stdlib_iterator_position, 128);
    benchmark!(c, space_stdlib_iterator_position, 256);
    benchmark!(c, space_stdlib_iterator_position, 512);
    benchmark!(c, space_stdlib_iterator_position, 1024);
    benchmark!(c, space_stdlib_iterator_position, 0x500000);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 64);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 128);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 256);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 512);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 1024);
    benchmark!(c, xml_delim_3_stdlib_find_char_set, 0x500000);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 64);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 128);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 256);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 512);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 1024);
    benchmark!(c, xml_delim_3_stdlib_find_char_closure, 0x500000);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 64);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 128);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 256);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 512);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 1024);
    benchmark!(c, xml_delim_3_stdlib_iterator_position, 0x500000);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 64);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 128);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 256);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 512);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 1024);
    benchmark!(c, xml_delim_5_stdlib_find_char_set, 0x500000);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 64);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 128);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 256);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 512);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 1024);
    benchmark!(c, xml_delim_5_stdlib_find_char_closure, 0x500000);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 64);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 128);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 256);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 512);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 1024);
    benchmark!(c, xml_delim_5_stdlib_iterator_position, 0x500000);
    benchmark!(c, substring_stdlib_find, 64);
    benchmark!(c, substring_stdlib_find, 128);
    benchmark!(c, substring_stdlib_find, 256);
    benchmark!(c, substring_stdlib_find, 512);
    benchmark!(c, substring_stdlib_find, 1024);
    benchmark!(c, substring_stdlib_find, 0x500000);
}

fn default_config() -> Criterion {
    Criterion::default()
        .significance_level(0.05)
        .sample_size(500)
        .measurement_time(Duration::from_secs(20))
}

criterion_group! {
    name = bench_stdlib;
    config = default_config();
    targets = do_bench
}

criterion_main!(bench_stdlib);
