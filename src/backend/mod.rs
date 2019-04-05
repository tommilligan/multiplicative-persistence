#[cfg(feature = "backend-num")]
pub mod backend_num_bigint;
#[cfg(feature = "backend-ramp")]
pub mod backend_ramp;

#[cfg(feature = "backend-num")]
pub use backend_num_bigint::multiplicative_persistence;
#[cfg(feature = "backend-ramp")]
pub use backend_ramp::multiplicative_persistence;

#[cfg(test)]
mod test {
    use super::*;

    // Over 64-bit integer
    const TOO_LARGE_INT: &'static str =
        "12346789123467891234678912346789123467891234678912346789123467891234678912346789123467891234678912346789123467891234678912346789";

    #[test]
    fn test_multiplicative_persistence() {
        assert_eq!(multiplicative_persistence("0"), 0);
        assert_eq!(multiplicative_persistence("3"), 0);
        assert_eq!(multiplicative_persistence("24"), 1);
        assert_eq!(multiplicative_persistence("39"), 3);
        assert_eq!(multiplicative_persistence(&TOO_LARGE_INT), 2);
    }

}
