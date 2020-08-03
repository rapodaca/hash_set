use std::collections::HashSet;
use libc::size_t;

#[no_mangle]
pub extern "C" fn hash_set_new() -> *mut HashSet<usize> {
    let result = HashSet::new();

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn hash_set_delete(set: *mut HashSet<usize>) {
    if set.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(set);
    }
}

#[no_mangle]
pub extern "C" fn hash_set_insert(
    set: *mut HashSet<usize>, item: size_t, result: *mut bool
) -> i8 {
    let set = match unsafe { set.as_mut() } {
        Some(set) => set,
        None => return -1
    };
    let result = match unsafe { result.as_mut() } {
        Some(result) => result,
        None => return -1
    };

    std::mem::replace(result, set.insert(item));
    
    0
}

#[no_mangle]
pub extern "C" fn hash_set_contains(
    set: *const HashSet<usize>, item: size_t, result: *mut bool
) -> i8 {
    let set = match unsafe { set.as_ref() } {
        Some(set) => set,
        None => return -1
    };
    let result = match unsafe { result.as_mut() } {
        Some(result) => result,
        None => return -1
    };

    std::mem::replace(result, set.contains(&item));

    0
}

#[no_mangle]
pub extern "C" fn hash_set_len(
    set: *const HashSet<usize>, result: *mut size_t
) -> i8 {
    let set = match unsafe { set.as_ref() } {
        Some(set) => set,
        None => return -1
    };
    let result = match unsafe { result.as_mut() } {
        Some(result) => result,
        None => return -1
    };

    std::mem::replace(result, set.len());

    0
}

#[no_mangle]
pub extern "C" fn hash_set_collect(
    set: *const HashSet<usize>, result: *mut size_t
) -> i8 {
    let set = match unsafe { set.as_ref() } {
        Some(set) => set,
        None => return -1
    };
    let items = set.iter().cloned().collect::<Vec<_>>();

    unsafe {
        std::ptr::copy((&items[0..]).as_ptr(), result, items.len());
    }

    0
}