pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for i in indices {
        if let Some(x) = slice.get_mut(*i) {
            *x = value;
        }
    }
}
