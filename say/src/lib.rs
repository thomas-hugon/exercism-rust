pub fn encode(n: u64) -> String {
    match n {
        n if n >= 1_000_000_000_000_000_000 => encode_suff(n, 1_000_000_000_000_000_000, " quintillion"),
        n if n >= 1_000_000_000_000_000 => encode_suff(n, 1_000_000_000_000_000, " quadrillion"),
        n if n >= 1_000_000_000_000 => encode_suff(n, 1_000_000_000_000, " trillion"),
        n if n >= 1_000_000_000 => encode_suff(n, 1_000_000_000, " billion"),
        n if n >= 1_000_000 => encode_suff(n, 1_000_000, " million"),
        n if n >= 1000 => encode_suff(n, 1000, " thousand"),
        n if n >= 100 => encode_suff(n, 100, " hundred"),
        n => encode_units((n % 100) / 10, n % 10),
    }
}

fn encode_suff(n: u64, th: u64, th_s: &'static str) -> String {
    match n % th {
        0 => encode(n / th) + th_s,
        modulo => encode(n / th) + th_s + " " +  &encode(modulo),
    }
}

fn encode_units(tenths: u64, units: u64) -> String {
    match (tenths, units) {
        (0, 0) => String::from("zero"),
        (0, 1) => String::from("one"),
        (0, 2) => String::from("two"),
        (0, 3) => String::from("three"),
        (0, 4) => String::from("four"),
        (0, 5) => String::from("five"),
        (0, 6) => String::from("six"),
        (0, 7) => String::from("seven"),
        (0, 8) => String::from("eight"),
        (0, 9) => String::from("nine"),
        (1, 0) => String::from("ten"),
        (1, 1) => String::from("eleven"),
        (1, 2) => String::from("twelve"),
        (1, 3) => String::from("thirteen"),
        (1, 4) => String::from("fourteen"),
        (1, 5) => String::from("fifteen"),
        (1, 6) => String::from("sixteen"),
        (1, 7) => String::from("seventeen"),
        (1, 8) => String::from("eighteen"),
        (1, 9) => String::from("nineteen"),
        (2, 0) => String::from("twenty"),
        (3, 0) => String::from("thirty"),
        (4, 0) => String::from("forty"),
        (5, 0) => String::from("fifty"),
        (6, 0) => String::from("sixty"),
        (7, 0) => String::from("seventy"),
        (8, 0) => String::from("eighty"),
        (9, 0) => String::from("ninety"),
        (a, b) => encode_units(a, 0) + "-" + &encode_units(0, b),
    }
}
