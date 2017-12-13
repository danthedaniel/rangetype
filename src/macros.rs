#[macro_export]
macro_rules! range {
    ($val:expr, $range:expr) => {
        {
            const_assert!($val >= $range.start && $val <= $range.end);
            RangeType::new($val, $range)
        }
    }
}
