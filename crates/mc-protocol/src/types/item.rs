//! Server-authored item stacks. Vanilla defaults stay in the client registry;
//! only the supported per-stack presentation overrides are sent on the wire.
use std::sync::Arc;

use futures_lite::AsyncWriteExt;

use crate::data::items::ITEMS;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

pub const MAX_TEXT_BYTES: usize = 256;
pub const MAX_LORE_LINES: usize = 16;
/// Enchantments per list, and custom effects per potion.
pub const MAX_ENCHANTMENTS: usize = 16;
pub const MAX_POTION_EFFECTS: usize = 16;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemComponents {
    pub damage: Option<u32>,
    /// None inherits the vanilla default; false explicitly removes the component.
    pub unbreakable: Option<bool>,
    pub name: Option<String>,
    pub lore: Vec<String>,
    pub dye: Option<u32>,
    pub glint: Option<bool>,
    /// (enchantment network ID, level) pairs, shown on the item without effect.
    pub enchantments: Vec<(u16, u8)>,
    /// Enchanted books' stored (enchantment network ID, level) pairs.
    pub stored_enchantments: Vec<(u16, u8)>,
    /// A potion's contents, shown and colored without drinking effects.
    pub potion: Option<PotionContents>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PotionContents {
    /// A `minecraft:potion` registry ID.
    pub potion: Option<u16>,
    pub color: Option<i32>,
    pub effects: Vec<PotionEffect>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PotionEffect {
    /// A `minecraft:mob_effect` registry ID.
    pub effect: u16,
    pub amplifier: u8,
    /// Ticks, or -1 for infinite.
    pub duration: i32,
    pub ambient: bool,
    pub particles: bool,
    pub icon: bool,
}

#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq, Eq)]
pub enum ItemError {
    #[error("unknown item or air")]
    InvalidItem,
    #[error("item count exceeds its vanilla stack limit")]
    InvalidCount,
    #[error("invalid or excessive item component data")]
    InvalidComponents,
}

/// Empty stacks allocate nothing. Nondefault patches are shared by snapshots,
/// split stacks and equipment; count changes never copy text or encode patches.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemStack {
    item_id: u16,
    count: u8,
    components: Option<Arc<ItemComponents>>,
}

impl ItemStack {
    pub fn new(item_id: i32, count: u8, components: ItemComponents) -> Result<Self, ItemError> {
        let id = usize::try_from(item_id).map_err(|_| ItemError::InvalidItem)?;
        let (_, max_stack, max_damage, _) = ITEMS.get(id).ok_or(ItemError::InvalidItem)?;
        if id == 0 {
            return Err(ItemError::InvalidItem);
        }
        if count == 0 || count > *max_stack {
            return Err(ItemError::InvalidCount);
        }
        if components.damage.is_some_and(|damage| *max_damage == 0 || damage > *max_damage)
            || components.name.as_ref().is_some_and(|name| name.len() > MAX_TEXT_BYTES)
            || components.lore.len() > MAX_LORE_LINES
            || components.lore.iter().any(|line| line.len() > MAX_TEXT_BYTES)
            || components.dye.is_some_and(|rgb| rgb > 0x00ff_ffff)
            || components.enchantments.len() > MAX_ENCHANTMENTS
            || components.stored_enchantments.len() > MAX_ENCHANTMENTS
            || components
                .enchantments
                .iter()
                .chain(&components.stored_enchantments)
                .any(|&(id, level)| usize::from(id) >= crate::data::item_registries::ENCHANTMENTS.len() || level == 0)
            || components.potion.as_ref().is_some_and(|potion| {
                potion.potion.is_some_and(|id| usize::from(id) >= crate::data::item_registries::POTIONS.len())
                    || potion.effects.len() > MAX_POTION_EFFECTS
                    || potion
                        .effects
                        .iter()
                        .any(|effect| usize::from(effect.effect) >= crate::data::item_registries::MOB_EFFECTS.len() || effect.duration < -1)
            })
            || components.body_len() > 4096
        {
            return Err(ItemError::InvalidComponents);
        }
        let components = (components != ItemComponents::default()).then(|| Arc::new(components));
        Ok(Self { item_id: id as u16, count, components })
    }

    pub fn plain(item_id: i32, count: u8) -> Result<Self, ItemError> {
        Self::new(item_id, count, ItemComponents::default())
    }

    pub fn item_id(&self) -> i32 {
        i32::from(self.item_id)
    }
    pub fn count(&self) -> u8 {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn components(&self) -> Option<&ItemComponents> {
        self.components.as_deref()
    }
    pub fn max_stack(&self) -> u8 {
        ITEMS[usize::from(self.item_id)].1
    }
    /// 0=ordinary, 1=feet, 2=legs, 3=chest, 4=head, 5=offhand.
    pub fn equipment_kind(&self) -> u8 {
        ITEMS[usize::from(self.item_id)].3
    }

    pub fn is_bundle(&self) -> bool {
        let name = ITEMS[usize::from(self.item_id)].0;
        name == "minecraft:bundle" || name.ends_with("_bundle")
    }

    pub fn same_item(&self, other: &Self) -> bool {
        !self.is_empty() && !other.is_empty() && self.item_id == other.item_id && self.components == other.components
    }

    pub fn take(&mut self, amount: u8) -> Self {
        let amount = amount.min(self.count);
        if amount == 0 {
            return Self::default();
        }
        let taken = Self { count: amount, ..self.clone() };
        self.count -= amount;
        if self.count == 0 {
            *self = Self::default();
        }
        taken
    }

    /// Transfer up to amount into an empty/compatible destination. The caller
    /// supplies its slot restriction, while item stack limits remain mandatory.
    pub fn transfer_to(&mut self, target: &mut Self, amount: u8, slot_limit: u8) -> u8 {
        if self.is_empty() || (!target.is_empty() && !self.same_item(target)) {
            return 0;
        }
        let limit = self.max_stack().min(slot_limit);
        let moved = amount.min(self.count).min(limit.saturating_sub(target.count));
        if moved == 0 {
            return 0;
        }
        if target.is_empty() {
            *target = self.take(moved);
        } else {
            target.count += moved;
            self.take(moved);
        }
        moved
    }
}

impl ItemComponents {
    fn added_count(&self) -> i32 {
        self.damage.is_some() as i32
            + (self.unbreakable == Some(true)) as i32
            + self.name.is_some() as i32
            + (!self.lore.is_empty()) as i32
            + self.dye.is_some() as i32
            + self.glint.is_some() as i32
            + (!self.enchantments.is_empty()) as i32
            + (!self.stored_enchantments.is_empty()) as i32
            + self.potion.is_some() as i32
    }

    fn body_len(&self) -> usize {
        let mut len = VarInt(self.added_count()).body_len() + 1;
        if let Some(value) = self.damage {
            len += 1 + VarInt(value as i32).body_len();
        }
        if self.unbreakable == Some(true) {
            len += 1;
        }
        if self.unbreakable == Some(false) {
            len += 1;
        }
        if let Some(name) = &self.name {
            len += 1 + Nbt::Str(name).body_len();
        }
        if !self.lore.is_empty() {
            len += 1 + VarInt(self.lore.len() as i32).body_len();
            len += self.lore.iter().map(|line| Nbt::Str(line).body_len()).sum::<usize>();
        }
        if self.glint.is_some() {
            len += 2;
        }
        if self.dye.is_some() {
            len += 5;
        }
        for list in [&self.enchantments, &self.stored_enchantments] {
            if !list.is_empty() {
                len += 1 + enchantments_len(list);
            }
        }
        if let Some(potion) = &self.potion {
            len += 1 + potion.body_len();
        }
        len
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        VarInt(self.added_count()).write(w).await?;
        VarInt((self.unbreakable == Some(false)) as i32).write(w).await?;
        if let Some(value) = self.damage {
            VarInt(3).write(w).await?;
            VarInt(value as i32).write(w).await?;
        }
        if self.unbreakable == Some(true) {
            VarInt(4).write(w).await?;
        }
        if let Some(name) = &self.name {
            VarInt(6).write(w).await?;
            Nbt::Str(name).write(w).await?;
        }
        if !self.lore.is_empty() {
            VarInt(11).write(w).await?;
            VarInt(self.lore.len() as i32).write(w).await?;
            for line in &self.lore {
                Nbt::Str(line).write(w).await?;
            }
        }
        if let Some(glint) = self.glint {
            VarInt(21).write(w).await?;
            glint.write(w).await?;
        }
        if let Some(rgb) = self.dye {
            VarInt(47).write(w).await?;
            (rgb as i32).write(w).await?;
        }
        if !self.enchantments.is_empty() {
            VarInt(13).write(w).await?;
            write_enchantments(&self.enchantments, w).await?;
        }
        if !self.stored_enchantments.is_empty() {
            VarInt(45).write(w).await?;
            write_enchantments(&self.stored_enchantments, w).await?;
        }
        if let Some(potion) = &self.potion {
            VarInt(53).write(w).await?;
            potion.write(w).await?;
        }
        if self.unbreakable == Some(false) {
            VarInt(4).write(w).await?;
        }
        Ok(())
    }
}

fn enchantments_len(list: &[(u16, u8)]) -> usize {
    VarInt(list.len() as i32).body_len()
        + list.iter().map(|&(id, level)| VarInt(id.into()).body_len() + VarInt(level.into()).body_len()).sum::<usize>()
}

/// A map of enchantment holder IDs to VarInt levels.
async fn write_enchantments<W: AsyncWriteExt + Unpin>(list: &[(u16, u8)], w: &mut W) -> crate::encode::Result {
    VarInt(list.len() as i32).write(w).await?;
    for &(id, level) in list {
        VarInt(id.into()).write(w).await?;
        VarInt(level.into()).write(w).await?;
    }
    Ok(())
}

impl PotionContents {
    fn body_len(&self) -> usize {
        1 + self.potion.map_or(0, |id| VarInt(id.into()).body_len())
            + 1
            + self.color.map_or(0, |_| 4)
            + VarInt(self.effects.len() as i32).body_len()
            + self
                .effects
                .iter()
                .map(|effect| {
                    VarInt(effect.effect.into()).body_len()
                        + VarInt(effect.amplifier.into()).body_len()
                        + VarInt(effect.duration).body_len()
                        + 4
                })
                .sum::<usize>()
            + 1
    }

    /// Optional potion, optional color, custom effects, then no custom name.
    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        self.potion.is_some().write(w).await?;
        if let Some(id) = self.potion {
            VarInt(id.into()).write(w).await?;
        }
        self.color.is_some().write(w).await?;
        if let Some(color) = self.color {
            color.write(w).await?;
        }
        VarInt(self.effects.len() as i32).write(w).await?;
        for effect in &self.effects {
            VarInt(effect.effect.into()).write(w).await?;
            VarInt(effect.amplifier.into()).write(w).await?;
            VarInt(effect.duration).write(w).await?;
            effect.ambient.write(w).await?;
            effect.particles.write(w).await?;
            effect.icon.write(w).await?;
            // No hidden weaker effect.
            false.write(w).await?;
        }
        false.write(w).await
    }
}

impl PacketWriter<'_> for ItemStack {
    fn read(_input: &mut &[u8]) -> Result<Self, DecodeError> {
        // Survival actions transmit hashed predictions, never authoritative stacks.
        Err(DecodeError::EnumOutOfRange)
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        VarInt(i32::from(self.count)).write(w).await?;
        if self.is_empty() {
            return Ok(());
        }
        VarInt(self.item_id()).write(w).await?;
        if let Some(components) = &self.components {
            components.write(w).await?;
        } else {
            w.write_all(&[0, 0]).await?;
        }
        Ok(())
    }

    fn body_len(&self) -> usize {
        if self.is_empty() {
            return 1;
        }
        1 + VarInt(self.item_id()).body_len() + self.components.as_deref().map_or(2, ItemComponents::body_len)
    }
}

#[cfg(test)]
#[path = "../tests/tests_types_item.rs"]
mod tests;
