use crate::plug::{Concrete, Unplug, Plug, forall_t};
use crate::classes::{Monoid, Functor, Applicative};


impl<A> Unplug for Option<A> {
    type F = Option<forall_t>;
    type A = A;
}

impl<A, B> Plug<A> for Option<B> {
    type result_t = Option<A>;
}

impl<A> Functor for Concrete<Option<forall_t>, A> {
    fn map<F, B>(f: F, s: Self) -> <Self as Plug<B>>::result_t
    where 
        F: FnMut(<Self as Unplug>::A) -> B + Clone,
    {
        let mapped_option = match s.unwrap {
            Some(value) => Some(f.clone()(value)),
            None => None,
        };

        Concrete::of(mapped_option)
    }
}

impl<A: Clone> Applicative for Concrete<Option<forall_t>, A> {
    fn pure(a: A) -> Self {
        Concrete::of(Some(a))
    }
    fn app<B, F>(fs: <Self as Plug<F>>::result_t, s: Self) -> <Self as Plug<B>>::result_t
    where  
        F: FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
        {
            let res = Functor::map(|x| Functor::map(|f| f.clone()(x.clone()), fs.clone()), s);
            let app_option = match res.unwrap {
                Some(value) => match value.unwrap {
                    Some(value2) => Some(value2), 
                    None => None,
                }
                None => None,
            };
            Concrete::of(app_option)
        }
}