use crate::cs::{PointType, self};

use super::{
    helper::sand_faling_helper,
    water::{BaseWater, SaltyWater, Water},
    CellRegistry, CellTrait, CellType, Prng, void::Void,
};

pub struct CrushedIce;
impl CrushedIce {
    pub const fn new() -> Self {
        Self
    }
    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
    pub fn id() -> CellType {
        56
    }
}
impl CellTrait for CrushedIce {
    fn update(
        &self,
        i: PointType,
        j: PointType,
        cur: usize,
        container: &mut [CellType],
        pal_container: &CellRegistry,
        prng: &mut Prng,
    ) {
        if sand_faling_helper(self.den(), i, j, container, pal_container, cur, prng) {
            return;
        }

        if prng.next() > 1 {
            return;
        }

        let top = cs::xy_to_index(i, j + 1);
        let bot = cs::xy_to_index(i, j - 1);
        let left = cs::xy_to_index(i + 1, j);
        let right = cs::xy_to_index(i - 1, j);

        let arr = [top, left, right, bot];
        let cc = arr[(prng.next() % 4) as usize];
        let rand_v = container[cc];

        if rand_v == Void::id() || rand_v == Water::id() {
            container[cur] = Water::id();
        }
    }

    fn stat(&self) -> bool {
        true
    }

    fn heatable(&self) -> CellType {
        Water::id()
    }

    fn heat_proof(&self) -> u8 {
        200
    }

    fn den(&self) -> i8 {
        5
    }
    fn id(&self) -> CellType {
        Self::id()
    }
    fn name(&self) -> String {
        "crushed_ice".to_owned()
    }
}