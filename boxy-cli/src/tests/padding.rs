#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn boxpad_uniform_all_sides_equal() {
        let p = BoxPad::uniform(3);
        assert_eq!((p.top, p.left, p.down, p.right), (3, 3, 3, 3));
    }

    #[test]
    fn boxpad_vh_correct_sides() {
        let p = BoxPad::vh(1, 4);
        assert_eq!((p.top, p.left, p.down, p.right), (1, 4, 1, 4));
    }

    #[test]
    fn boxpad_lr_sum() {
        let p = BoxPad::from_tldr(0, 3, 0, 5);
        assert_eq!(p.lr(), 8);
    }

    #[test]
    fn boxpad_from_tldr_argument_order() {
        let p = BoxPad::from_tldr(1, 2, 3, 4);
        assert_eq!((p.top, p.left, p.down, p.right), (1, 2, 3, 4));
    }

    #[test]
    fn boxpad_new_all_zeros() {
        let p = BoxPad::new();
        assert_eq!((p.top, p.left, p.down, p.right), (0, 0, 0, 0));
    }
}
