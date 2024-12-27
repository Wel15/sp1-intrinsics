//! bn254 scalar operation

/// `BN254_SCALAR_MUL` syscall ID.
pub const BN254_SCALAR_MUL: u32 = 0x00_01_01_80;

/// `BN254_SCALAR_MAC` syscall ID.
pub const BN254_SCALAR_MAC: u32 = 0x00_01_01_81;

/// `BN254_SCALAR_MULADD` syscall ID.
pub const BN254_SCALAR_MULADD: u32 = 0x00_01_01_1F;

/// Perform in-place scalar multiplication `p *= q`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `p` must be [valid] for writes of [`bn254::Fr`], and must remain valid even
///   when `q` is read for [`bn254::Fr`].
///
/// * `q` must be [valid] for reads of [`bn254::Fr`].
///
/// * Both `p` and `q` must be properly aligned and not overlap.
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_mul<P, Q>(p: *mut P, q: *const Q) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MUL, p, q)
    }
}

/// Perform in-place scalar multiplication and addition `ret += a + b`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `ret` must be [valid] for writes of [`bn254::Fr`], and must remain valid even
///   when `a` and `b` are read for [`bn254::Fr`].
///
/// * `a` and `b` must be [valid] for reads of [`bn254::Fr`].
///
/// * Both `ret`, `a`, and `b` must be properly aligned and not overlap.
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_mac<R, T>(ret: *mut R, a: *const T, b: *const T) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MAC, ret, &[a, b])
    }
}

/// Perform scalar multiplication and addition `ret = a * b + c`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `ret` must be [valid] for writes of [`bn254::Fr`]
/// * `a`, `b`, and `c` must be [valid] for reads of [`bn254::Fr`]
/// * All pointers must be properly aligned and not overlap
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_muladd(
    ret: *mut [u32; 8],
    a: *const [u32; 8],
    b: *const [u32; 8],
    c: *const [u32; 8],
) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MULADD, ret, a, b, c)
    }
}

// If you need a generic version, you can also add this alternative implementation:
/*
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_muladd_generic<T>(
    ret: *mut T,
    a: *const T,
    b: *const T,
    c: *const T,
) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MULADD, ret, a, b, c)
    }
}
*/