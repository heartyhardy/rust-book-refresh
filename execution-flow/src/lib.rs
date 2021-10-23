#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
        let i = 20;
        // Rust's if statement does not require parenthesis
        if i < 2 {
            assert!(i < 2);
        } else if i > 2 {
            assert!(i > 2);
        } else {
            assert_eq!(i, 2);
        }
    }

    #[test]
    fn more_conditionals() {
        let my_option = Some(10);
        // If let statements can do simple pattern matching
        if let Some(unpacked) = my_option {
            assert_eq!(unpacked, 10);
        }
        let mut other_option = Some(2);
        // there is also while let, which does the same thing
        while let Some(unpacked) = other_option {
            // if can also return values in assignments
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else {
                None
            }
        }
        assert_eq!(other_option, None)
    }

    #[test]
    fn loops() {
        let mut i = 42;
        let mut broke = false;
        // a basic loop with control statements
        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        assert!(broke);

        'outer: loop {
            'inner: loop {
                break 'inner; // ... and specifically jumped out of
            }
            break 'outer;
        }
    }
}
