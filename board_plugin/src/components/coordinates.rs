//Coordinate is a component ! it's only data


use bevy::prelude::Component;
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

#[cfg_attr(feature="debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartielEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinantes {
    type Output = self;

    fn add(self, rhs: Self) -> Self::Output {
        self {
            x: self.x + rhs.x,
            y: self.y + rhx.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = self;

    fn sub(self, rhs: Self) -> Self::Output {
        self {
            x: self.x.saturatong_sub(rhs.x), //avoid panic if negatif result
            y: self.y.saturating_sub(rhx.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}