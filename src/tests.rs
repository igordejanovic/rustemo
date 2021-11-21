#[cfg(test)]
pub(crate) mod utils {
    pub(crate) fn type_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    pub(crate) fn string_difference(a: &String, b: &String) -> Option<(usize, (char, char))> {
        Some(
            a.chars()
                .zip(b.chars())
                .enumerate()
                .find(|(_, (a, b))| a != b)?,
        )
    }
}
