#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::global::logger().info($msg, &[]);
    };
    ($msg:expr, $($key:ident = $value:expr), * $(,)?) => {
        $crate::global::logger().info($msg, &[$((stringify!($key), &$value as &dyn std::fmt::Debug)), *])
    };
}
