#[macro_export]
macro_rules! sig {
    // 基本型（例：I32, String など）
    ($ty:ident) => {
        CoreType::$ty
    };

    // タプル型
    (($($elem:tt),*)) => {
        CoreType::Tuple(vec![$(sig!{$elem}),*])
    };

    // 関数型
    ($arg:tt -> $ret:tt) => {
        CoreType::Fn(Box::new(sig!{$arg}), Box::new(sig!{$ret}))
    };
}
