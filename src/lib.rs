#![allow(dead_code)]
extern crate rand;
mod tests;

use rand::{Rng, SeedableRng, StdRng};

trait Item: Eq + Clone {}

struct LootTable<T: Item> {
    rng: StdRng,
    sum_of_chances: u32,
    chances: Vec<u32>,
    items: Vec<T>,
}

impl<T> LootTable<T>
where
    T: Item,
{
    pub fn from_equal_chance(items: &[T], seed: &[usize]) -> LootTable<T> {
        let item_count = items.iter().count();
        let chances = vec![1 as u32; item_count];
        LootTable {
            rng: SeedableRng::from_seed(seed),
            sum_of_chances: item_count as u32,
            chances: chances,
            items: items.to_vec(),
        }
    }
    pub fn chance_for(&self, item: &T) -> f32 {
        let position = self.items.iter().position(|i| i == item);
        match position {
            Some(idx) => self.chances[idx] as f32 / self.sum_of_chances as f32,
            None => 0.,
        }
    }
    pub fn roll(&mut self) -> &T {
        let choice = self.rng.gen::<u32>() % self.sum_of_chances;

        let mut accumulator = 0;
        for (idx, item) in self.items.iter().enumerate() {
            accumulator += self.chances[idx];
            if accumulator > choice {
                return item;
            }
        }

        panic!("loot table is empty");
    }
}
