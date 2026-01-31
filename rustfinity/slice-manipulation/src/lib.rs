pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for i in indices.iter() {
        if *i >= slice.len() {
            continue;
        } else {
            slice[*i] = value;
        }
    }
}
