use std::collections::HashMap;

pub fn damlev(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    let inf = a_len + b_len;
    let mut m = Vec::new();
    m.reserve(a_len + 2);
    m.push(vec![inf; b_len + 2]);

    let mut tmp = vec![inf];
    tmp.extend(0..(b_len + 1));
    m.push(tmp);

    for i in 1..(a_len + 1) {
        let mut tmp = vec![inf, i];
        tmp.append(&mut vec![0; b_len]);
        m.push(tmp);
    }

    let mut lr = HashMap::new();
    let mut a_chars = a.chars();

    for i in 1..(a_len + 1) {
        let c_a = a_chars.next().unwrap();
        let mut lmc = 0;
        let mut b_chars = b.chars();

        for j in 1..(b_len + 1) {
            let c_b = b_chars.next().unwrap();
            let lmr = *lr.get(&c_b).unwrap_or(&0);
            let cost = if c_b == c_a { 0 } else { 1 };
            let tr_cost = std::cmp::max(i - lmr - 1, j - lmc - 1) + 1;

            m[i + 1][j + 1] = *(&[
                m[i][j] + cost,
                m[i + 1][j] + 1,
                m[i][j + 1] + 1,
                m[lmr][lmc] + tr_cost,
            ]).iter().min().unwrap();

            if cost == 0 {
                lmc = j;
            }
        }

        lr.insert(c_a, i);
    }

    *m.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damerau_levenshtein_empty() {
        assert_eq!(0, damlev("", ""));
    }

    #[test]
    fn damerau_levenshtein_same() {
        assert_eq!(0, damlev("damerau", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_first_empty() {
        assert_eq!(7, damlev("", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_second_empty() {
        assert_eq!(7, damlev("damerau", ""));
    }

    #[test]
    fn damerau_levenshtein_diff() {
        assert_eq!(2, damlev("ca", "abc"));
    }

    #[test]
    fn damerau_levenshtein_diff_short() {
        assert_eq!(3, damlev("damerau", "aderua"));
    }

    #[test]
    fn damerau_levenshtein_diff_reversed() {
        assert_eq!(3, damlev("aderua", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_multibyte() {
        assert_eq!(3, damlev("öঙ香", "abc"));
        assert_eq!(3, damlev("abc", "öঙ香"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length() {
        assert_eq!(6, damlev("damerau", "aderuaxyz"));
    }

    #[test]
    fn damerau_levenshtein_diff_unequal_length_reversed() {
        assert_eq!(6, damlev("aderuaxyz", "damerau"));
    }

    #[test]
    fn damerau_levenshtein_diff_comedians() {
        assert_eq!(5, damlev("Stewart", "Colbert"));
    }

    #[test]
    fn damerau_levenshtein_many_transpositions() {
        assert_eq!(4, damlev("abcdefghijkl", "bacedfgihjlk"));
    }

    #[test]
    fn damerau_levenshtein_diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(36, damlev(a, b));
    }

    #[test]
    fn damerau_levenshtein_beginning_transposition() {
        assert_eq!(1, damlev("foobar", "ofobar"));
    }

    #[test]
    fn damerau_levenshtein_end_transposition() {
        assert_eq!(1, damlev("specter", "spectre"));
    }

    #[test]
    fn damerau_levenshtein_unrestricted_edit() {
        assert_eq!(3, damlev("a cat", "an abct"));
    }
}
