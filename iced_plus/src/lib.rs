use iced::Padding;

pub trait PaddingExtensions {
    fn expand(self, extra: Padding) -> Padding;
    fn shrink(self, reduction: Padding) -> Padding;
}

impl PaddingExtensions for Padding {
    fn expand(self, extra: Padding) -> Padding {
        Padding {
            top: self.top + extra.top,
            right: self.right + extra.right,
            bottom: self.bottom + extra.bottom,
            left: self.left + extra.left,
        }
    }

    fn shrink(self, reduction: Padding) -> Padding {
        Padding {
            top: 0f32.max(self.top - reduction.top),
            right: 0f32.max(self.right - reduction.right),
            bottom: 0f32.max(self.bottom - reduction.bottom),
            left: 0f32.max(self.left - reduction.left),
        }
    }
}

#[cfg(test)]
mod test {
    use iced::Padding;

    use crate::PaddingExtensions;

    #[test]
    pub fn expanding_is_independent() {
        let input = Padding{ top: 1f32, right: 2f32, bottom: 3f32, left: 4f32 };
        let extra = Padding{ top: 10f32, right: 20f32, bottom: 30f32, left: 40f32 };
        let result = input.expand(extra);
        assert_eq!(11f32, result.top);
        assert_eq!(22f32, result.right);
        assert_eq!(33f32, result.bottom);
        assert_eq!(44f32, result.left);
    }

    #[test]
    pub fn shrinking_is_independent() {
        let input = Padding{ top: 12f32, right: 24f32, bottom: 36f32, left: 48f32 };
        let reduction = Padding{ top: 1f32, right: 2f32, bottom: 3f32, left: 4f32 };
        let result = input.shrink(reduction);
        assert_eq!(11f32, result.top);
        assert_eq!(22f32, result.right);
        assert_eq!(33f32, result.bottom);
        assert_eq!(44f32, result.left);
    }

    #[test]
    pub fn shrinking_doesnt_go_below_zero() {
        let input = Padding{ top: 1f32, right: 2f32, bottom: 3f32, left: 4f32 };
        let reduction = Padding{ top: 12f32, right: 24f32, bottom: 36f32, left: 48f32 };
        let result = input.shrink(reduction);
        assert_eq!(0f32, result.top);
        assert_eq!(0f32, result.right);
        assert_eq!(0f32, result.bottom);
        assert_eq!(0f32, result.left);
    }
}