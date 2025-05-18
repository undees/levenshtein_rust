use magnus::{function, prelude::*, Error, RString};

/// Compute the Levenshtein edit distance between two UTF-8 strings.
#[inline]
fn distance(s1: &str, s2: &str) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    if s1_len == 0 || s2_len == 0 {
        return s1_len.max(s2_len);
    }

    if s1 == s2 {
        return 0;
    }

    let mut distances: Vec<usize> = (0..=s2_len).collect();
    let mut workspace: Vec<usize> = vec![0; s2_len + 1];

    for (i, c1) in s1.chars().enumerate() {
        workspace[0] = i + 1;

        for (j, c2) in s2.chars().enumerate() {
            let deletion_cost = distances[j + 1] + 1;
            let insertion_cost = workspace[j] + 1;
            let substitution_cost = distances[j] +
                (if c1 == c2 { 0 } else { 1 });

            workspace[j + 1] = deletion_cost
                .min(insertion_cost)
                .min(substitution_cost);
        }

        std::mem::swap(&mut distances, &mut workspace);
    }

    // SAFETY: `distances` is guaranteed to be non-empty because `s2_len > 0`.
    *distances.last().unwrap()
}

fn distance_ruby(s1: RString, s2: RString) -> Result<usize, magnus::Error> {
    let s1: String = s1.try_convert()?; // validates UTF-8
    let s2: String = s2.try_convert()?;

    Ok(distance(&s1, &s2))
}


#[magnus::init]
fn init() -> Result<(), Error> {
    let module = magnus::define_module("LevenshteinRust")?;
    module.define_singleton_method("distance", function!(distance_ruby, 2))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::distance;

    #[test]
    fn test_assert_emptiness() {
        assert_eq!(distance("", ""), 0);
        assert_eq!(distance("a", ""), 1);
        assert_eq!(distance("", "a"), 1);
    }

    #[test]
    fn test_assert_edits() {
        assert_eq!(distance("abc", "azc"), 1);
        assert_eq!(distance("abc", "acb"), 2);
        assert_eq!(distance("abc", "ac" ), 1);
        assert_eq!(distance("ac",  "abc"), 1);
    }

    #[test]
    fn test_utf8() {
        assert_eq!(distance("la", "là"), 1);
    }
}
