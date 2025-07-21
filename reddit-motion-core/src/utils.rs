use std::fmt::Debug;

#[derive(Default)]
pub(crate) struct VecCollectionDebug<'a, T: std::fmt::Debug>(&'a [T]);

impl<'a, T: std::fmt::Debug> From<&'a [T]> for VecCollectionDebug<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self(value)
    }
}

impl<'a, T: Debug> Debug for VecCollectionDebug<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
