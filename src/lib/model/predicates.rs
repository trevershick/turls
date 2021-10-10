use predicates::prelude::*;

use super::SearchParams;
use super::Shortened;

struct AlwaysTrue;
struct TermContains {
    value: String,
}
impl predicates::reflection::PredicateReflection for AlwaysTrue {}
impl predicates::reflection::PredicateReflection for TermContains {}
impl std::fmt::Display for AlwaysTrue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "always true")
    }
}
impl std::fmt::Display for TermContains {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "any term contains")
    }
}
impl predicates::Predicate<Shortened> for TermContains {
    fn eval(&self, var: &Shortened) -> bool {
        use predicates::str::contains;
        let c = contains(self.value.clone());
        c.eval(&var.url)
            || c.eval(&var.keyword)
            || c.eval(var.title.clone().unwrap_or("".to_owned()).as_ref())
    }
}

impl<T> predicates::Predicate<T> for AlwaysTrue {
    fn eval(&self, _var: &T) -> bool {
        true
    }
}
impl From<&SearchParams> for predicates::BoxPredicate<Shortened> {
    fn from(s: &SearchParams) -> predicates::BoxPredicate<Shortened> {
        let mut root: predicates::BoxPredicate<Shortened> = AlwaysTrue {}.boxed();
        if s.term.len() > 0 {
            root = root
                .and(TermContains {
                    value: s.term.to_owned(),
                })
                .boxed();
        }
        root.boxed()
    }
}
