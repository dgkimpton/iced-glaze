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