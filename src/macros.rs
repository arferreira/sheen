#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::global::logger().info($msg, &[]);
    };
    ($msg:expr, $($key:ident = $value:expr), * $(,)?) => {
        $crate::global::logger().info($msg, &[$((stringify!($key), &$value as &dyn std::fmt::Debug)), *])
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::global::logger().debug($msg, &[])
    };
    ($msg:expr, $($key:ident = $value:expr),* $(,)?) => {
        $crate::global::logger().debug($msg, &[$(( stringify!($key), &$value as &dyn std::fmt::Debug )),*])
    };
}

#[macro_export]
macro_rules! trace {
    ($msg:expr) => {
        $crate::global::logger().trace($msg, &[])
    };
    ($msg:expr, $($key:ident = $value:expr),* $(,)?) => {
        $crate::global::logger().trace($msg, &[$(( stringify!($key), &$value as &dyn std::fmt::Debug )),*])
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::global::logger().warn($msg, &[])
    };
    ($msg:expr, $($key:ident = $value:expr),* $(,)?) => {
        $crate::global::logger().warn($msg, &[$(( stringify!($key), &$value as &dyn std::fmt::Debug )),*])
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::global::logger().error($msg, &[])
    };
    ($msg:expr, $($key:ident = $value:expr),* $(,)?) => {
        $crate::global::logger().error($msg, &[$(( stringify!($key), &$value as &dyn std::fmt::Debug )),*])
    };
}
