pub fn compute_indices(
    idx: usize,
    l: usize,
    r: usize,
) -> (usize, usize, usize) {
    let m = (l + r) / 2;
    let lc = idx + 1;
    let rc = idx + (m - l + 1) * 2;
    (lc, rc, m)
}
