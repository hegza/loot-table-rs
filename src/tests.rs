use super::*;
#[cfg(test)]
use Item;

#[derive(Clone, PartialEq, Eq)]
struct TestItem(i32);
impl Item for TestItem {}

#[test]
fn it_works() {
    let items = vec![TestItem(0), TestItem(1), TestItem(2)];
    let table = LootTable::from_equal_chance(&items, &[1, 2, 3, 4]);

    let chance_for_0 = table.chance_for(&items[0]);
    let chance_for_1 = table.chance_for(&items[1]);
    let chance_for_2 = table.chance_for(&items[2]);

    const EPSILON: f32 = 0.001;
    assert!((chance_for_0 - (1. / 3.)).abs() < EPSILON);
    assert!((chance_for_1 - (1. / 3.)).abs() < EPSILON);
    assert!((chance_for_2 - (1. / 3.)).abs() < EPSILON);
}
