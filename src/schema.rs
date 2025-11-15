pub trait Schema {
    fn get(&self) -> isize;
}

impl Schema for isize {
    fn get(&self) -> isize {
        *self
    }
}

// Cheap dependency-free way to have a bounded integer
#[derive(Debug, Clone, Copy)]
pub enum BoundedSchema {
    Neg4 = -4,
    Neg3 = -3,
    Neg2 = -2,
    Neg1 = -1,
    Pos0 = 0,
    Pos1 = 1,
    Pos2 = 2,
    Pos3 = 3,
    Pos4 = 4,
    Pos5 = 5,
    Pos6 = 6,
    Pos7 = 7,
    Pos8 = 8,
}

impl TryFrom<isize> for BoundedSchema {
    type Error = isize;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            -4 => Ok(BoundedSchema::Neg4),
            -3 => Ok(BoundedSchema::Neg3),
            -2 => Ok(BoundedSchema::Neg2),
            -1 => Ok(BoundedSchema::Neg1),
            0 => Ok(BoundedSchema::Pos0),
            1 => Ok(BoundedSchema::Pos1),
            2 => Ok(BoundedSchema::Pos2),
            3 => Ok(BoundedSchema::Pos3),
            4 => Ok(BoundedSchema::Pos4),
            5 => Ok(BoundedSchema::Pos5),
            6 => Ok(BoundedSchema::Pos6),
            7 => Ok(BoundedSchema::Pos7),
            8 => Ok(BoundedSchema::Pos8),
            _ => Err(value),
        }
    }
}

impl Schema for BoundedSchema {
    fn get(&self) -> isize {
        *self as isize
    }
}
