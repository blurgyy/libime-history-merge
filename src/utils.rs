pub(crate) fn gcd(a: u8, b: u8) -> u8 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub(crate) fn split_vec<T: Clone>(v: Vec<T>, sizes: &[usize]) -> Vec<Vec<T>> {
    let mut ret = Vec::new();

    let mut v = v;
    for size in sizes {
        let size = std::cmp::min(v.len(), *size);
        log::trace!("appending {}-sized vec", size);
        if size == 0 {
            break;
        }
        ret.push(v[0..size].to_vec());
        v = v[size..].to_vec();
    }

    ret
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 05 2022, 15:41 [CST]
