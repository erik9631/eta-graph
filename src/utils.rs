pub fn split_to_parts<T>(input: &[T], parts: usize) -> Vec<&[T]>{
    let (quot, rem) = (input.len() / parts, input.len() % parts);

    let mut parts_vec = Vec::new();
    unsafe {
        let mut input_ptr = input.as_ptr();
        for _ in 0..parts {
            let part = std::slice::from_raw_parts(input_ptr, quot);
            parts_vec.push(part);
            input_ptr = input_ptr.offset(quot as isize);
        }

        if rem > 0 {
            let part = std::slice::from_raw_parts(input_ptr, rem);
            parts_vec.push(part);
        }
    }
    return parts_vec;

}

pub fn split_to_mut_parts<T>(input: &mut [T], parts: usize) -> Vec<&mut [T]>{
    let (quot, rem) = (input.len() / parts, input.len() % parts);

    let mut parts_vec = Vec::new();
    unsafe {
        let mut input_ptr = input.as_mut_ptr();
        for _ in 0..parts {
            let part = std::slice::from_raw_parts_mut(input_ptr, quot);
            parts_vec.push(part);
            input_ptr = input_ptr.offset(quot as isize);
        }

        if rem > 0 {
            let part = std::slice::from_raw_parts_mut(input_ptr, rem);
            parts_vec.push(part);
        }
    }
    return parts_vec;

}