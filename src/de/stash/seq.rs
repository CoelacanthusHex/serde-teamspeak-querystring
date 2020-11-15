use serde::de;

use super::map::PairMap;
use crate::de::Deserializer;
use crate::error::{Error, Result};

pub(crate) enum ItemKind<'de> {
    Value(&'de [u8]),
    Map(PairMap<'de>),
}

pub(crate) struct PairSeq<'de> {
    items: Vec<ItemKind<'de>>,
    remaining_depth: u16,
}

impl<'de> PairSeq<'de> {
    pub(crate) fn new(items: Vec<ItemKind<'de>>, remaining_depth: u16) -> Self {
        Self {
            items,
            remaining_depth,
        }
    }
}

impl<'de> de::SeqAccess<'de> for PairSeq<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.items.pop() {
            Some(ItemKind::Value(value)) => seed
                .deserialize(&mut Deserializer::new_with_depth(
                    value,
                    self.remaining_depth,
                ))
                .map(Some),
            Some(ItemKind::Map(map)) => seed.deserialize(map).map(Some),
            None => Ok(None),
        }
    }
}
