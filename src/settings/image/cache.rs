use core::mem;

use crate::platform::prelude::*;
use hashbrown::{HashTable, hash_table::Entry};
use slab::Slab;

use super::{Image, ImageId};

/// A trait for types that have an image ID. This is used for the [`ImageCache`]
/// to look up images by their ID.
pub trait HasImageId {
    /// Returns the image ID of the object.
    fn image_id(&self) -> &ImageId;
}

impl HasImageId for Image {
    fn image_id(&self) -> &ImageId {
        &self.id
    }
}

/// A cache for images that allows looking up images by their ID. The cache uses
/// a garbage collection algorithm to remove images that have not been visited
/// since the last garbage collection. The cache is generic over the type of
/// image it stores, so you may use it to store textures or image URLs as well.
/// Functions updating the cache usually don't run the garbage collection
/// themselves, so make sure to call [`collect`](Self::collect) every now and
/// then to remove unvisited images.
pub struct ImageCache<T = Image> {
    table: HashTable<Key>,
    elements: Slab<Element<T>>,
    bitvec_visited: Vec<u64>,
    newest: Key,
    oldest: Key,
}

struct Element<T> {
    value: T,
    newer: Key,
    older: Key,
}

type Key = usize;
const KEY_NONE: Key = Key::MAX;

// Data Structure:
//
// All images are stored in a slab and have a unique key that identifies them.
// The elements in the slab form a doubly linked list with each element pointing
// to a newer and an older element. The linked list is specifically ordered to
// form an LRU cache with the most recently used elements being on one side and
// the least recently used elements on the other. We also have a hash table that
// can quickly look up an image by its image ID, which is a strong hash. The
// hash table cuts out 64-bit of the strong hash as the hash for the table. The
// table then maps that image ID to a key to look up the actual element in the
// slab. The moment it gets visited the element gets reattached to be the newest
// element in the doubly linked list. We also have a bit vector that stores
// which elements have been visited with the key being used to determine the bit
// position in the bit vector. To garbage collect we first determine how many
// elements to collect based on the fill rate of the bit vector. We then drain
// elements from the least recently used side of the doubly linked list and
// reset the bit vector to 0.

impl<T: HasImageId> Default for ImageCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasImageId> ImageCache<T> {
    /// Creates a new image cache.
    pub const fn new() -> Self {
        Self {
            table: HashTable::new(),
            elements: Slab::new(),
            bitvec_visited: Vec::new(),
            newest: KEY_NONE,
            oldest: KEY_NONE,
        }
    }

    /// Looks up an image in the cache based on its image ID. If the image is
    /// not in the cache, [`None`] is returned. This does not mark the image as
    /// visited.
    pub fn lookup(&self, image_id: &ImageId) -> Option<&T> {
        self.table
            .find(image_id.hash(), |image_key| {
                self.elements[*image_key].value.image_id() == image_id
            })
            .map(|image_key| &self.elements[*image_key].value)
    }

    /// Caches an image based on its image ID. If the image is already in the
    /// cache, it is returned. Otherwise, the image is built and inserted into
    /// the cache. This marks the image as visited regardless of whether it was
    /// already in the cache or not.
    pub fn cache(&mut self, image_id: &ImageId, build: impl FnOnce() -> T) -> &mut T {
        let element_key = match self.table.entry(
            image_id.hash(),
            |image_key| self.elements[*image_key].value.image_id() == image_id,
            |image_key| self.elements[*image_key].value.image_id().hash(),
        ) {
            Entry::Occupied(v) => {
                let element_key = *v.into_mut();
                let element = &mut self.elements[element_key];

                let before_key = mem::replace(&mut element.older, KEY_NONE);
                let after_key = mem::replace(&mut element.newer, KEY_NONE);

                if before_key != KEY_NONE {
                    self.elements[before_key].newer = after_key;
                } else {
                    self.oldest = after_key;
                }

                if after_key != KEY_NONE {
                    self.elements[after_key].older = before_key;
                } else {
                    self.newest = before_key;
                }

                element_key
            }
            Entry::Vacant(v) => {
                let element_key = self.elements.insert(Element {
                    value: build(),
                    newer: KEY_NONE,
                    older: KEY_NONE,
                });

                v.insert(element_key);

                let new_len = element_key / 64 + 1;
                if new_len > self.bitvec_visited.len() {
                    self.bitvec_visited.resize(new_len, 0);
                }

                element_key
            }
        };

        self.bitvec_visited[element_key / 64] |= 1 << (element_key % 64);

        let before_key = mem::replace(&mut self.newest, element_key);
        if before_key != KEY_NONE {
            self.elements[before_key].newer = element_key;
        } else {
            self.oldest = element_key;
        }

        let element = &mut self.elements[element_key];
        element.older = before_key;
        &mut element.value
    }

    /// Runs the garbage collection of the cache. This removes images from the
    /// cache that have not been visited since the last garbage collection. Not
    /// every image that has not been visited is removed. There is a heuristic
    /// that keeps a certain amount of images in the cache regardless of whether
    /// they have been visited or not. Returns the amount of images that got
    /// collected.
    pub fn collect(&mut self) -> usize {
        // The strategy is to allow twice the amount of images in the cache than
        // the ones that are currently actively still being used + 5 extra so
        // the strategy is still useful at a low amount of images.
        let total_collect_count = self
            .table
            .len()
            .saturating_sub(2 * self.visited_count() + 5);

        if total_collect_count > 0 {
            let mut current_oldest = self.oldest;
            for _ in 0..total_collect_count {
                let removed = self.elements.remove(current_oldest);
                self.table
                    .find_entry(removed.value.image_id().hash(), |&image_key| {
                        image_key == current_oldest
                    })
                    .unwrap()
                    .remove();
                current_oldest = removed.newer;
            }
            self.oldest = current_oldest;
            self.elements[self.oldest].older = KEY_NONE;
        }

        self.bitvec_visited.fill(0);

        total_collect_count
    }

    fn visited_count(&self) -> usize {
        self.bitvec_visited
            .iter()
            .map(|v| v.count_ones() as usize)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyImage(ImageId);

    impl HasImageId for MyImage {
        fn image_id(&self) -> &ImageId {
            &self.0
        }
    }

    /// Validates that the linked list is formed correctly.
    #[track_caller]
    fn assert_consistency<T>(cache: &ImageCache<T>) {
        if cache.newest == KEY_NONE || cache.oldest == KEY_NONE {
            assert_eq!(cache.newest, cache.oldest);
            return;
        }
        let mut current = cache.oldest;
        let mut previous = KEY_NONE;
        assert_eq!(cache.elements[current].older, KEY_NONE);
        for _ in 0..1000 {
            assert_eq!(cache.elements[current].older, previous);
            let next = cache.elements[current].newer;
            if next == KEY_NONE {
                assert_eq!(cache.newest, current);
                return;
            }
            previous = current;
            current = next;
        }
        panic!("Entered a cycle.");
    }

    #[test]
    fn collects_as_expected() {
        let mut image_cache = ImageCache::new();

        for i in 0..20 {
            let id = ImageId([i; 32]);
            image_cache.cache(&id, || MyImage(id));
            assert_consistency(&image_cache);
        }

        assert_eq!(image_cache.visited_count(), 20);
        assert_eq!(image_cache.table.len(), 20);

        image_cache.collect();
        assert_consistency(&image_cache);

        assert_eq!(image_cache.visited_count(), 0);
        assert_eq!(image_cache.table.len(), 20);

        for i in 0..3 {
            let id = ImageId([i; 32]);
            image_cache.cache(&id, || MyImage(id));
            assert_consistency(&image_cache);
        }

        assert_eq!(image_cache.visited_count(), 3);
        assert_eq!(image_cache.table.len(), 20);

        image_cache.collect();
        assert_consistency(&image_cache);

        // Only 3 images out of the 20 were visited, so because we allow two times
        // the visited images plus 5 images to be in the cache, we should see 2 * 3
        // + 5 = 11 images left.
        assert_eq!(image_cache.visited_count(), 0);
        assert_eq!(image_cache.table.len(), 11);

        image_cache.collect();
        assert_consistency(&image_cache);

        // 2 * 0 + 5 = 5
        assert_eq!(image_cache.visited_count(), 0);
        assert_eq!(image_cache.table.len(), 5);
    }

    #[test]
    fn single() {
        let mut image_cache = ImageCache::new();
        assert_consistency(&image_cache);

        let id = ImageId([0; 32]);
        image_cache.cache(&id, || MyImage(id));
        assert_consistency(&image_cache);
        image_cache.cache(&id, || MyImage(id));
        assert_consistency(&image_cache);
        image_cache.collect();
        assert_consistency(&image_cache);
        image_cache.cache(&id, || MyImage(id));
        assert_consistency(&image_cache);
        image_cache.collect();
    }
}
