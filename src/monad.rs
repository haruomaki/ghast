pub trait Functor {
    type A;
    type Lifted<B>: Functor;

    // fn map<F, B>(self, f: F) -> Self::Lifted<B>
    // where
    //     F: FnMut(Self::A) -> B;
}

pub trait Pointed: Functor {
    // fn pure(t: Self::A) -> Self::Lifted<Self::A>;
}

pub trait Applicative: Pointed {
    // fn apply<F, B, C>(self, b: Self::Lifted<B>, f: F) -> Self::Lifted<C>
    // where
    //     F: FnMut(Self::A, B) -> C;
}

pub trait Monad: Functor {
    fn bind<B, F>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> Self::Lifted<B> + 'static;

    fn ret(a: Self::A) -> Self
    where
        Self::A: Clone;
}

// https://blog-dry.com/entry/2020/12/25/130250#do-記法
#[macro_export]
macro_rules! mdo {
    ($i:ident <- $e:expr; $($t:tt)*) => {
        $e.bind(move |$i| mdo!($($t)*))
    };
    ($e:expr; $($t:tt)*) => {
        $e.bind(move |()| mdo!($($t)*))
    };
    (=> $e:expr) => {
        Monad::ret($e)
    };
    ($e:expr) => {
        $e
    };
}
