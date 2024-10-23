// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

#[allow(dead_code)]
struct DropBomb {
    should_panic: bool,
}

#[allow(dead_code)]
impl DropBomb {
    fn new() -> Self {
        Self {
            should_panic: true,
        }
    }

    fn defuse(&mut self) {
        self.should_panic = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.should_panic {
            panic!("Boom");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
