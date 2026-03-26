use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::fmt::Debug;

/// All the indexed apps that rustcast can search for
#[derive(Default)]
pub struct Index<T: Searchable + Clone> {
    items: Vec<T>,
    matcher: SkimMatcherV2,
}

impl<T: Searchable + Clone + Debug> std::fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Index")
            .field("items", &self.items)
            .finish_non_exhaustive()
    }
}

/// A trait for any item that can be put into an [`Index`]
pub trait Searchable {
    fn alias(&self) -> String;
}

impl<T: Searchable + Clone> Index<T> {
    /// Creates a new index. Uses [`SkimMatcherV2::default`] for initing the matcher.
    pub fn new(items: &[T]) -> Self {
        let matcher = SkimMatcherV2::default();

        Index {
            items: items.to_vec(),
            matcher,
        }
    }

    /// Creates a new index with a provided [`SkimMatcherV2`].
    pub fn new_with_matcher(items: &[T], matcher: SkimMatcherV2) -> Self {
        Index {
            items: items.to_vec(),
            matcher,
        }
    }

    /// Adds a single item to the index
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    /// Adds multiple items to the index from a slice
    pub fn add_from_slice(&mut self, items: &[T]) {
        self.items.extend_from_slice(items);
    }

    /// Runs a fuzzy search on the index.
    ///
    /// Each item in the vec is a pair of `(score, value)`
    pub fn search(&self, query: &str) -> Vec<(i64, T)> {
        let mut scored_results: Vec<(i64, T)> = self
            .items
            .iter()
            .filter_map(|item| {
                self.matcher
                    .fuzzy_match(&item.alias(), query)
                    .map(|score| (score, item.clone()))
            })
            .collect();

        scored_results.sort_by(|a, b| b.0.cmp(&a.0));
        scored_results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Searchable for &str {
        fn alias(&self) -> String {
            self.to_string()
        }
    }

    #[test]
    fn test_fuzzy_search() {
        // Guess this mostly just tests skim in practice, but shut

        let index = Index::new(&["firefox", "fifox"]);

        let results: Vec<_> = index.search("ffox").iter().map(|x| x.1).collect();

        assert_eq!(results, ["fifox", "firefox"]);
    }

    #[test]
    fn test_insertion() {
        // Guess this is actually a slightly useless test

        let mut index = Index::default();
        index.add("a");
        index.add_from_slice(&["b", "c"]);

        assert_eq!(index.items, vec!["a", "b", "c"]);
    }
}
