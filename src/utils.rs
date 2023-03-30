// Compare two iterators to check if the needle is contained in the haystack.
// Because iteration has to be restarted, the caller must provide two factory
// functions that can skip to the provided offset.
pub fn contains_kmp<I: Iterator<Item=char> + Clone>(left: I, mut right: I) -> bool {
    let mut left = left.peekable();

    // Consume the first needle character
    let first = match right.next() {
        None => return true,
        Some(c) => c
    };

    let right_restart = right.clone();

    let mut restart: Option<core::iter::Peekable<I>>;
    
    loop {
        // Consume left until a char matches the 1st needle char
        let x = match left.next() {
            None => return false,
            Some(c) => c
        };

        if x != first {
            continue;
        }

        restart = None;

        loop {
            let y = match right.next() {
                None => return true, // end of needle -> we have found a match
                Some(c) => c
            };

            // If the next char in haystack is a starting char, and we haven't seen
            // a starting char already, clone the iterator in its current state so
            // we can use it as a restart point.
            if let Some(nxt) = left.peek() {
                if *nxt == first && restart.is_none() {
                    restart = Some(left.clone());
                }
            }

            let x = match left.next() {
                None => return false, // end of haystack -> we still have some needle left
                Some(c) => c
            };

            if y == x {
                continue;
            }

            // We need to resume iteration at the relevant starting point. Haystack
            // from the first starting character we saw or its current point, and
            // needle from the 2nd character in the string.
            left = restart.unwrap_or(left);
            right = right_restart.clone();

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