// Compare two iterators to check if the needle is contained in the haystack.
// Because iteration has to be restarted, the caller must provide two factory
// functions that can skip to the provided offset.
pub fn contains_kmp<F1, F2, I: Iterator<Item=char> + Clone>(haystack: F1, needle: F2) -> bool
where F1 : Fn(usize) -> I, F2 : Fn(usize) -> I {
    let mut left = haystack(0);
    let mut right = needle(0);

    // Consume the first needle character
    let first = match right.next() {
        None => return true,
        Some(c) => c
    };
    
    let mut next_possible_start: usize;
    let mut i = 0;

    loop {
        // Consume left until a char matches the 1st needle char
        let x = match left.next() {
            None => return false,
            Some(c) => c
        };

        i += 1;

        if x != first {
            continue;
        }

        let mut ix = 0;
        next_possible_start = 0;

        loop {
            let y = match right.next() {
                None => return true, // end of needle -> we have found a match
                Some(c) => c
            };

            let x = match left.next() {
                None => return false, // end of haystack -> we still have some needle left
                Some(c) => c
            };

            ix += 1;

            if x == first && next_possible_start == 0 {
                next_possible_start = ix;
            }

            if y == x {
                continue;
            }

            // Overwrite the two iterators, needle from the top (except the first char)
            // and haystack from the last seen starting point of a potential match
            left = haystack(i + next_possible_start - 1);
            right = needle(1);

            break;
        }
    }
}

// Compare two iterators, returning true if they are equivalent or
// the value of `if_right_empty` when the end of the right iterator is reached.
#[inline]
pub fn compare_iters<I1: Iterator<Item=char>, I2: Iterator<Item=char>>(left: &mut I1, right: &mut I2, if_right_empty: bool) -> bool {
    loop {
        let x = match left.next() {
            None => return right.next().is_none(),
            Some(val) => val,
        };

        let y = match right.next() {
            None => return if_right_empty,
            Some(val) => val,
        };

        if x != y {
            return false;
        }
    }
}