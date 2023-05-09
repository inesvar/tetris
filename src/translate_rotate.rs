use crate::point::Point;
pub struct TranslateRotate {
    pub translation: Point,
    pub rotation: i8,
    pub center: Option<Point>,
}

impl TranslateRotate {
    pub fn new(translation: Point, rotation: i8, center: &Point) -> Self {
        let center = *center + translation;
        TranslateRotate {
            translation,
            rotation,
            center: Some(center),
        }
    }

    pub fn translation(translation: Point) -> Self {
        TranslateRotate { translation, rotation: 0, center: None }
    }

    pub fn fall() -> Self {
        TranslateRotate::translation(Point::new(0, 1))
    }

    pub fn right() -> Self {
        TranslateRotate::translation(Point::new(1, 0))
    }

    pub fn left() -> Self {
        TranslateRotate::translation(Point::new(-1, 0))
    }
}