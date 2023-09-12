use crate::parser::Parser;

pub fn single(expected: char) -> Parser<char> {
    Parser::single(expected)
}
pub fn chunk(expected: impl AsRef<str> + 'static) -> Parser<()> {
    Parser::chunk(expected)
}
pub fn ascii_digit() -> Parser<char> {
    Parser::satisfy(|c| char::is_ascii_digit(&c))
}
pub fn digit(radix: u32) -> Parser<char> {
    Parser::satisfy(move |c| char::is_digit(c, radix))
}
pub fn numeric() -> Parser<char> {
    Parser::satisfy(char::is_numeric)
}

// https://blog-dry.com/entry/2020/12/25/130250#do-記法
#[macro_export]
macro_rules! pdo {
    ($($t:tt)*) => {
        monapa::pdo_with_env!{~~ $($t)*}
    };
}

#[macro_export]
macro_rules! pdo_with_env {
    // 値を取り出してbindする（>>=）
    (~$($env:ident)*~ $i:ident <- $e:expr; $($t:tt)*) => {
        $(let $env = $env.clone();)*
        monapa::Parser::bind($e, move |$i| {monapa::pdo_with_env!{~$($env)* $i~ $($t)*}})
    };

    // モナドから取り出した値を使わない場合（>>）
    (~$($env:ident)*~ $e:expr; $($t:tt)*) => {
        $(let $env = $env.clone();)*
        monapa::Parser::bind($e, move |_| {monapa::pdo_with_env!{~$($env)*~ $($t)*}})
    };

    // return関数
    (~$($env:ident)*~ return $e:expr) => {
        $(let $env = $env.clone();)*
        monapa::Parser::ret($e)
    };

    // returnでなくモナドを直接指定して返す
    (~$($env:ident)*~ $e:expr) => {
        $(let $env = $env.clone();)*
        $e
    };
}
