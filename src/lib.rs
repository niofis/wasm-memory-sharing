#[no_mangle]
pub unsafe fn wasm_alloc_f32(n: usize) -> *mut f32 {
    let mut memory = Vec::with_capacity(n);
    let ptr = memory.as_mut_ptr();
    std::mem::forget(memory);
    ptr
}

#[no_mangle]
pub unsafe fn free_f32_vec(n: usize, ptr: *mut f32) {
    let _bytes: Vec<f32> = Vec::from_raw_parts(ptr, n, n);
}

#[no_mangle]
pub unsafe fn multiply_f32_v(n: usize, output: *mut f32, input_a: *mut f32, input_b: *mut f32) {
    let mut out: Vec<f32> = Vec::from_raw_parts(output, n, n);
    let a: Vec<f32> = Vec::from_raw_parts(input_a, n, n);
    let b: Vec<f32> = Vec::from_raw_parts(input_b, n, n);
    a.iter()
        .zip(b.iter())
        .zip(out.iter_mut())
        .for_each(|((a, b), out)| {
            *out = a * b;
        });
    std::mem::forget(out);
    std::mem::forget(a);
    std::mem::forget(b);
}
