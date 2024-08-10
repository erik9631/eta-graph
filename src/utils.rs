use std::alloc::{alloc, Layout};

pub fn split_to_parts<T>(input: &[T], number_of_parts: usize) -> Vec<&[T]>{
    let (quot, rem) = (input.len() / number_of_parts, input.len() % number_of_parts);

    let mut parts_vec = Vec::new();
    unsafe {
        let mut input_ptr = input.as_ptr();
        for _ in 0..number_of_parts {
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

pub fn split_to_parts_mut<T>(input: &mut [T], number_of_parts: usize) -> Vec<&mut [T]>{
    let (quot, rem) = (input.len() / number_of_parts, input.len() % number_of_parts);

    let mut parts_vec = Vec::new();
    unsafe {
        let mut input_ptr = input.as_mut_ptr();
        for _ in 0..number_of_parts {
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

pub fn extract_from_slice<T>(slice: &[T], start: usize, size: usize) -> (&[T], &[T], &[T]){
    unsafe{
        let begin_ptr = slice.as_ptr();
        let data_start_ptr = slice.as_ptr().offset(start as isize);
        let data_end_ptr = data_start_ptr.offset(size as isize);
        let end_part_size = slice.len() - start + size;

        let first_part = std::slice::from_raw_parts(begin_ptr, start);
        let mid_part = std::slice::from_raw_parts(data_start_ptr, size);
        let end_part = std::slice::from_raw_parts(data_end_ptr, end_part_size);
        return (first_part, mid_part, end_part);
    }
}

pub fn extract_from_slice_mut<T>(slice: &mut [T], start: usize, size: usize) -> (&mut [T], &mut [T], &mut [T]){
    unsafe{
        let begin_ptr = slice.as_mut_ptr();
        let data_start_ptr = slice.as_mut_ptr().offset(start as isize);
        let data_end_ptr = data_start_ptr.offset(size as isize);
        let end_part_size = slice.len() - start + size;

        let first_part = std::slice::from_raw_parts_mut(begin_ptr, start);
        let mid_part = std::slice::from_raw_parts_mut(data_start_ptr, size);
        let end_part = std::slice::from_raw_parts_mut(data_end_ptr, end_part_size);
        return (first_part, mid_part, end_part);
    }
}

pub fn alloc_array<Type>(size: usize) -> (&'static mut[Type], Layout){
    let layout = Layout::array::<Type>(size).expect("Failed to create layout");
    let ptr = unsafe { alloc(layout) };
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut Type, size) };
    return (slice, layout);
}

pub fn dealloc_array<Type>(slice: (&'static mut[Type], Layout)){
    unsafe { std::alloc::dealloc(slice.0.as_mut_ptr() as *mut u8, slice.1) };
}
