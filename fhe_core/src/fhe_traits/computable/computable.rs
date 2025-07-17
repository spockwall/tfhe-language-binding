use tfhe::{FheInt64, FheUint64};

/// Define Computable trait for arithmetic operations.
///
/// FheType that implements Computable trait can be used in arithmetic.
/// Input and output are the same FheType.
///
/// Supported FheType: "FheInt", "FheUint"
pub trait Computable {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn xor(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
    fn not(&self) -> Self;
}

/// Define Shiftable trait for shift operations
///
/// FheType that implements Shiftable trait can be used in arithmetic.
/// Input and output are the same FheType.
///
/// Supported FheType: "FheUint"
pub trait Shiftable {
    fn shr(&self, other: &Self) -> Self;
    fn shl(&self, other: &Self) -> Self;
}

/// Implement Computable trait using macro_rules
///
/// Input FheType: FheInt, FheUint
/// Output FheType: FheInt, FheUint
/// Example:
/// ```no_run  
/// impl_computable!(FheUint64);
/// ```
macro_rules! impl_computable {
    ($fhe_ty:ty) => {
        impl Computable for $fhe_ty {
            fn add(&self, other: &Self) -> Self {
                self + other
            }
            fn sub(&self, other: &Self) -> Self {
                self - other
            }
            fn mul(&self, other: &Self) -> Self {
                self * other
            }
            fn div(&self, other: &Self) -> Self {
                self / other
            }
            fn rem(&self, other: &Self) -> Self {
                self % other
            }
            fn and(&self, other: &Self) -> Self {
                self & other
            }
            fn or(&self, other: &Self) -> Self {
                self | other
            }
            fn xor(&self, other: &Self) -> Self {
                self ^ other
            }
            fn neg(&self) -> Self {
                -self
            }
            fn not(&self) -> Self {
                !self
            }
        }
    };
}

/// Implement Shiftable trait use macro_rules
///
/// Input FheType: FheUint
/// Output FheType: FheUint
/// Example:
///
/// ```no_run
/// impl_shiftable!(FheUint64);
/// ```
macro_rules! impl_shiftable {
    ($fhe_ty:ty) => {
        impl Shiftable for $fhe_ty {
            fn shr(&self, other: &Self) -> Self {
                self >> other
            }
            fn shl(&self, other: &Self) -> Self {
                self << other
            }
        }
    };
}

impl_computable!(FheInt64);
impl_computable!(FheUint64);
impl_shiftable!(FheUint64);
