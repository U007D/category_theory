#[cfg(test)]
mod unit_tests;

#[must_use]
pub const fn identity<T>(item: T) -> T { item }