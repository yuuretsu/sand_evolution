use crate::cs::PointType;

use super::{gas::Gas, helper::sand_faling_helper, CellRegistry, CellTrait, CellType, Prng, water::{BaseWater, SaltyWater}};

pub struct Sand;
impl Sand {
    pub const fn new() -> Self {
        Self
    }
    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
    pub fn id() -> CellType {
        1
    }
}
impl CellTrait for Sand {
    fn update(
        &self,
        i: PointType,
        j: PointType,
        cur: usize,
        container: &mut [CellType],
        pal_container: &CellRegistry,
        prng: &mut Prng,
    ) {
        sand_faling_helper(self.den(), i, j, container, pal_container, cur, prng);
    }
    fn den(&self) -> i8 {
        10
    }
    fn proton_transfer(&self) -> CellType {
        Gas::id()
    }
    fn id(&self) -> CellType {
        1
    }
    fn name(&self) -> String {
        "sand".to_owned()
    }
}

pub struct Salt;
impl Salt {
    pub const fn new() -> Self {
        Self
    }
    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
    pub fn id() -> CellType {
        13
    }
}
impl CellTrait for Salt {
    fn update(
        &self,
        i: PointType,
        j: PointType,
        cur: usize,
        container: &mut [CellType],
        pal_container: &CellRegistry,
        prng: &mut Prng,
    ) {
        sand_faling_helper(self.den(), i, j, container, pal_container, cur, prng);
    }
    fn den(&self) -> i8 {
        10
    }
    fn dissolve(&self) -> CellType {
        SaltyWater::id()
    }
    fn id(&self) -> CellType {
        13
    }
    fn name(&self) -> String {
        "salt".to_owned()
    }
}


pub struct Base;
impl Base {
    pub const fn new() -> Self {
        Self
    }
    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
    pub fn id() -> CellType {
        14
    }
}
impl CellTrait for Base {
    fn update(
        &self,
        i: PointType,
        j: PointType,
        cur: usize,
        container: &mut [CellType],
        pal_container: &CellRegistry,
        prng: &mut Prng,
    ) {
        sand_faling_helper(self.den(), i, j, container, pal_container, cur, prng);
    }
    fn den(&self) -> i8 {
        10
    }
    fn proton_transfer(&self) -> CellType {
        Salt::id()
    }
    fn dissolve(&self) -> CellType {
        BaseWater::id()
    }
    fn id(&self) -> CellType {
        14
    }
    fn name(&self) -> String {
        "base".to_owned()
    }
}