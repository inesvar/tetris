use super::point::Point;
pub struct TranslationRotation {
    pub translation: Point,
    pub rotation: i8,
    pub center: Option<Point>,
}

impl TranslationRotation {
    pub fn new(translation: Point, rotation: i8, center: &Point) -> Self {
        let center = *center + translation;
        TranslationRotation {
            translation,
            rotation,
            center: Some(center),
        }
    }

    pub fn null() -> Self {
        TranslationRotation {
            translation: Point::new(0, 0),
            rotation: 0,
            center: None,
        }
    }

    pub fn translation(translation: Point) -> Self {
        TranslationRotation {
            translation,
            rotation: 0,
            center: None,
        }
    }

    pub fn fall() -> Self {
        TranslationRotation::translation(Point::new(0, 1))
    }

    pub fn right() -> Self {
        TranslationRotation::translation(Point::new(1, 0))
    }

    pub fn left() -> Self {
        TranslationRotation::translation(Point::new(-1, 0))
    }
}
