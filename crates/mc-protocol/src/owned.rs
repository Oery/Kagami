use std::borrow::Cow;

use uuid::Uuid;

/// Detaches a decoded value from the buffer it borrows, so it can outlive
/// the read, for example by moving into a spawned task.
///
/// Derive it with `mc_protocol_macros::IntoOwned`; packets get it from
/// `#[packet]`.
pub trait IntoOwned {
    type Owned: 'static;

    fn into_owned(self) -> Self::Owned;
}

impl IntoOwned for Cow<'_, str> {
    type Owned = Cow<'static, str>;

    fn into_owned(self) -> Cow<'static, str> {
        Cow::Owned(Cow::into_owned(self))
    }
}

impl IntoOwned for Cow<'_, [u8]> {
    type Owned = Cow<'static, [u8]>;

    fn into_owned(self) -> Cow<'static, [u8]> {
        Cow::Owned(Cow::into_owned(self))
    }
}

impl<T: IntoOwned> IntoOwned for Vec<T> {
    type Owned = Vec<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        self.into_iter().map(T::into_owned).collect()
    }
}

impl<T: IntoOwned, const N: usize> IntoOwned for [T; N] {
    type Owned = [T::Owned; N];

    fn into_owned(self) -> Self::Owned {
        self.map(T::into_owned)
    }
}

impl<T: IntoOwned> IntoOwned for Box<T> {
    type Owned = Box<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        Box::new((*self).into_owned())
    }
}

impl<T: IntoOwned> IntoOwned for Option<T> {
    type Owned = Option<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        self.map(T::into_owned)
    }
}

impl<A: IntoOwned, B: IntoOwned> IntoOwned for (A, B) {
    type Owned = (A::Owned, B::Owned);

    fn into_owned(self) -> Self::Owned {
        (self.0.into_owned(), self.1.into_owned())
    }
}

/// Implements `IntoOwned` as the identity for types that borrow nothing.
macro_rules! already_owned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoOwned for $ty {
                type Owned = Self;

                fn into_owned(self) -> Self {
                    self
                }
            }
        )*
    };
}

already_owned!(bool, u8, i8, u16, i16, i32, i64, f32, f64, String, Uuid, crate::VarInt, crate::types::VarLong);
