use bevy::prelude::Color;
use bevy_rapier2d::prelude::Collider;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fruit {
    Cherry,
    Strawberry,
    Grape,
    Decopon,
    Persimmon,
    Apple,
    Pear,
    Peach,
    Pineapple,
    Melon,
    Watermelon,
}

impl Fruit {
    pub fn collider(&self) -> Collider {
        Collider::ball(self.radius())
    }

    pub fn radius(&self) -> f32 {
        let base = 10.0;
        let multiplier: f32 = 1.3;
        match self {
            Fruit::Cherry => base * multiplier,
            Fruit::Strawberry => base * multiplier.powi(2),
            Fruit::Grape => base * multiplier.powi(3),
            Fruit::Decopon => base * multiplier.powi(4),
            Fruit::Persimmon => base * multiplier.powi(5),
            Fruit::Apple => base * multiplier.powi(6),
            Fruit::Pear => base * multiplier.powi(7),
            Fruit::Peach => base * multiplier.powi(8),
            Fruit::Pineapple => base * multiplier.powi(9),
            Fruit::Melon => base * multiplier.powi(10),
            Fruit::Watermelon => base * multiplier.powi(11),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Fruit::Cherry => Color::rgb(0.9375, 0.0, 0.0),
            Fruit::Strawberry => Color::rgb(0.9375, 0.42578125, 0.33203125),
            // 147, 73, 255
            Fruit::Grape => Color::rgb(0.57421875, 0.28515625, 1.0),
            // 252, 151, 9
            Fruit::Decopon => Color::rgb(0.98828125, 0.58984375, 0.03515625),
            // 248, 113, 20
            Fruit::Persimmon => Color::rgb(0.9765625, 0.0, 0.0),
            // 236, 0, 20
            Fruit::Apple => Color::rgb(0.92578125, 0.0, 0.0),
            // 249, 242, 102
            Fruit::Pear => Color::rgb(0.9765625, 0.9765625, 0.40234375),
            // 252, 194, 189
            Fruit::Peach => Color::rgb(0.98828125, 0.7578125, 0.73828125),
            // 236, 234, 15
            Fruit::Pineapple => Color::rgb(0.92578125, 0.91796875, 0.05859375),
            // 117, 199, 22
            Fruit::Melon => Color::rgb(0.45703125, 0.78125, 0.0859375),
            // 20, 87, 16
            Fruit::Watermelon => Color::rgb(0.078125, 0.33984375, 0.0625),
        }
    }

    pub fn promote(&self, other: &Self) -> Option<Self> {
        if self == other {
            match self {
                Fruit::Cherry => Some(Fruit::Strawberry),
                Fruit::Strawberry => Some(Fruit::Grape),
                Fruit::Grape => Some(Fruit::Decopon),
                Fruit::Decopon => Some(Fruit::Persimmon),
                Fruit::Persimmon => Some(Fruit::Apple),
                Fruit::Apple => Some(Fruit::Pear),
                Fruit::Pear => Some(Fruit::Peach),
                Fruit::Peach => Some(Fruit::Pineapple),
                Fruit::Pineapple => Some(Fruit::Melon),
                Fruit::Melon => Some(Fruit::Watermelon),
                Fruit::Watermelon => None,
            }
        } else {
            None
        }
    }
}
