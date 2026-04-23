pub mod debug {
    use crate::syntax::Span;
    use std::fmt::Debug;

    impl Debug for Span {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("SourceRef")
                .field(&self.start)
                .field(&self.end)
                .finish()
        }
    }
}

pub mod deref {
    use {
        crate::syntax::Spanned,
        std::ops::{Deref, DerefMut},
    };

    impl<T> Deref for Spanned<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.node
        }
    }

    impl<T> DerefMut for Spanned<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.node
        }
    }

    impl<T> Spanned<T> {
        pub fn map<F, U>(self, f: F) -> Spanned<U>
        where
            F: FnOnce(T) -> U,
        {
            Spanned {
                node: f(self.node),
                span: self.span,
            }
        }
    }
}
