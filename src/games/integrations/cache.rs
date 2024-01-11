use tinyvec::TinyVec;
use once_cell::sync::Lazy;

// Struct holds cache for some method per each game and its edition
const CACHE_SIZE: usize = 64;

#[derive(Default)]
pub struct Cache<Key: Default, Value: Default> {
    cache: TinyVec<[(Key, Value); CACHE_SIZE]>
}

impl<Key: Default + Eq, Value: Default> Cache<Key, Value> {
    #[inline]
    pub const fn lazy_new() -> Lazy<Self> {
        Lazy::new(Self::default)
    }

    #[inline(always)]
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.cache.iter()
            .find(|(k, _)| key.eq(k))
            .map(|(_, v)| v)
    }

    #[inline]
    pub fn set(&mut self, key: Key, value: Value) {
        for (k, v) in &mut self.cache {
            if key.eq(k) {
                *v = value;

                return;
            }
        }

        self.cache.push((key, value));
    }
}
