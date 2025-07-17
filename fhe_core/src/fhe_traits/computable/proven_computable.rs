pub use tfhe::zk::CompactPkePublicParams as CPPP;
pub use tfhe::{
    CompactPublicKey as CPK, FheInt64, FheUint64, ProvenCompactFheInt64, ProvenCompactFheUint64,
};

/// Define ProvenComputable trait for arithmetic operations using zk.
///
/// Proven fhe type that implements ProvenComputable trait can be used in
/// arithmetic with zk feature. A client can encrypt and prove a ciphertext
/// and a server can validate the encrypted value before conducting the
/// operation by using public zk params and public key passed.
///
/// Supported FheType: "ProvenFheInt", "ProvenFheUint"
pub trait ProvenComputable {
    type Output;

    fn add(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn sub(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn mul(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn div(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn rem(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn and(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn or(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn xor(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn neg(&self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn not(&self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
}

/// Define ProvenShiftable trait for shift operations using zk.
///
/// Proven fhe type that implements ProvenShiftable trait can be used in
/// arithmetic with zk feature. A client can encrypt and prove a ciphertext
/// and a server can validate the encrypted value before conducting the operation
/// by using public zk params and public key passed.
///
/// Supported Fhe Type: "ProvenFheUint"
pub trait ProvenShiftable {
    type Output;
    fn shr(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
    fn shl(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output;
}

/// Implement ProvenComputable trait using macro_rules
///
/// Input FheType: ProvenFheInt, ProvenFheUint
/// Output FheType: FheInt, FheUint
/// Example:
/// ```no_run
/// impl_proven_computable!(ProvenCompactFheUint64, FheUint64);
/// ```
macro_rules! impl_proven_computable {
    ($proven_fhe_ty:ty, $fhe_ty:ty) => {
        impl ProvenComputable for $proven_fhe_ty {
            type Output = $fhe_ty;
            fn add(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() + b.unwrap()
            }
            fn sub(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() - b.unwrap()
            }
            fn mul(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() * b.unwrap()
            }
            fn div(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() / b.unwrap()
            }
            fn rem(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() % b.unwrap()
            }
            fn and(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() & b.unwrap()
            }
            fn or(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() | b.unwrap()
            }
            fn xor(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() ^ b.unwrap()
            }
            fn neg(&self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                -a.unwrap()
            }
            fn not(&self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                !a.unwrap()
            }
        }
    };
}

/// Implement ProvenShiftable trait using macro_rules
///
/// Input FheType: ProvenFheUint
/// Output FheType: FheUint
/// Example:
/// ```no_run
/// impl_proven_shiftable!(ProvenCompactFheUint64, FheUint64);
/// ```
macro_rules! impl_proven_shiftable {
    ($proven_fhe_ty:ty, $fhe_ty:ty) => {
        impl ProvenShiftable for $proven_fhe_ty {
            type Output = $fhe_ty;
            fn shr(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() >> b.unwrap()
            }
            fn shl(&self, o: &Self, public_zk_params: &CPPP, public_key: &CPK) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() << b.unwrap()
            }
        }
    };
}

impl_proven_computable!(ProvenCompactFheUint64, FheUint64);
impl_proven_computable!(ProvenCompactFheInt64, FheInt64);
impl_proven_shiftable!(ProvenCompactFheUint64, FheUint64);
