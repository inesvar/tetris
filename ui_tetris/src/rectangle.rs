use graphics::rectangle;
use graphics::types;
use graphics::types::Scalar;

#[derive(Clone)]
pub(super) struct Rectangle {
    pub(super) center_x: Scalar,
    pub(super) center_y: Scalar,
    pub(super) width: Scalar,
    pub(super) height: Scalar,
}

impl Rectangle {
    pub(super) fn new(center_x: Scalar, center_y: Scalar, width: Scalar, height: Scalar) -> Self {
        Rectangle {
            center_x,
            center_y,
            width,
            height,
        }
    }

    pub(super) fn contains(&self, &[x, y]: &[Scalar; 2]) -> bool {
        x >= self.center_x - self.width / 2.0
            && x <= self.center_x + self.width / 2.0
            && y >= self.center_y - self.height / 2.0
            && y <= self.center_y + self.height / 2.0
    }

    pub(super) fn get_dimensions(&self) -> types::Rectangle {
        rectangle::centered([0.0, 0.0, self.width / 2.0, self.height / 2.0])
    }

    pub(super) fn get_center(&self) -> (Scalar, Scalar) {
        (self.center_x, self.center_y)
    }
}
