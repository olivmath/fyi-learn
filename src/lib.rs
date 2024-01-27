use std::slice;
use tiny_keccak::{Hasher, Keccak};

pub fn hash_it(data: &[u8], result: &mut [u8; 32]) {
    let mut k256 = Keccak::v256();

    k256.update(data);
    k256.finalize(result);
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn run1() {
    println!("RUST SIDE: Hello FFI");
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn run2(leaves_ptr: *const *const u8, len_leaves: usize) {
    let leaves = unsafe { slice::from_raw_parts(leaves_ptr, len_leaves) }
        .iter()
        .map(|leaf_ptr| unsafe { slice::from_raw_parts(*leaf_ptr, 32).to_vec() })
        .collect::<Vec<Vec<u8>>>();

    for leaf in leaves {
        println!("RUST SIDE: {leaf:?}");
    }
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn run3(leaves_ptr: *mut *mut u8, len_leaves: usize) {
    let leaves: Vec<Vec<u8>> = unsafe { slice::from_raw_parts(leaves_ptr, len_leaves) }
        .iter()
        .map(|leaf_ptr| unsafe { slice::from_raw_parts(*leaf_ptr, 32).to_vec() })
        .collect();

    for leaf in leaves {
        let mut result: [u8; 32] = [0; 32];
        hash_it(&leaf, &mut result);
        println!("RUST SIDE: {result:?}");
    }
}

type Callback1 = extern "C" fn(leaf: *const u8, buffer: *mut u8);
/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn run4(callback: Callback1, leaves_ptr: *mut *mut u8, len_leaves: usize) {
    let leaves: Vec<Vec<u8>> = unsafe { slice::from_raw_parts(leaves_ptr, len_leaves) }
        .iter()
        .map(|leaf_ptr| unsafe { slice::from_raw_parts(*leaf_ptr, 32).to_vec() })
        .collect();

    for leaf in &leaves {
        println!("RUST WITHOUT HASH: {leaf:?}");
    }
    for leaf in leaves {
        let mut result: [u8; 32] = [0; 32];
        callback(leaf.as_ptr(), result.as_mut_ptr());
        println!("RUST WITH HASH: {result:?}");
    }
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn hash_within_rust(leaves_ptr: *mut *mut u8, len_leaves: usize) {
    let leaves: Vec<Vec<u8>> = unsafe { slice::from_raw_parts(leaves_ptr, len_leaves) }
        .iter()
        .map(|leaf_ptr| unsafe { slice::from_raw_parts(*leaf_ptr, 32).to_vec() })
        .collect();

    for leaf in leaves {
        let mut result: [u8; 32] = [0; 32];
        hash_it(&leaf, &mut result);
        println!("RUST SIDE: {result:?}");
    }
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn hash_without_rust(
    callback: Callback1,
    leaves_ptr: *mut *mut u8,
    len_leaves: usize,
) {
    let leaves: Vec<Vec<u8>> = unsafe { slice::from_raw_parts(leaves_ptr, len_leaves) }
        .iter()
        .map(|leaf_ptr| unsafe { slice::from_raw_parts(*leaf_ptr, 32).to_vec() })
        .collect();

    for leaf in &leaves {
        println!("RUST WITHOUT HASH: {leaf:?}");
    }
    for leaf in leaves {
        let mut result: [u8; 32] = [0; 32];
        callback(leaf.as_ptr(), result.as_mut_ptr());
        println!("RUST WITH HASH: {result:?}");
    }
}

/// # Safety
///
/// FFI to Python.
#[no_mangle]
pub unsafe extern "C" fn run5(leaves_ptr: *const *const u8, len_leaves: usize) -> *const u8 {
    let mut hasher = Keccak::v256();
    let mut final_hash = [0u8; 32];

    for i in 0..len_leaves {
        let leaf = slice::from_raw_parts(*leaves_ptr.add(i), 32);
        hasher.update(leaf);
    }

    hasher.finalize(&mut final_hash);

    let boxed_hash = Box::new(final_hash);
    Box::into_raw(boxed_hash) as *const u8
}
