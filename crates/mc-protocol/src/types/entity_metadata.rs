//! Entity metadata: the synced values that drive an entity's look and state,
//! such as its flags, pose, custom name, held item or variant.
use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::tagged::tagged_enum;
use crate::types::{GlobalPos, IdOr, Nbt, Particle, ResolvableProfile, Slot, VarInt, VarLong};

/// Ends the entries on the wire.
const END_OF_ENTRIES: u8 = 0xFF;

/// Metadata entries, each an index into the entity's synced values and a
/// typed value. Only the values that changed are usually sent.
#[derive(Clone, Debug, Default, PartialEq, IntoOwned)]
pub struct EntityMetadata<'p>(pub Vec<MetadataEntry<'p>>);

impl<'p> EntityMetadata<'p> {
    /// The value at `index`, if this update carries it.
    pub fn get(&self, index: u8) -> Option<&MetadataValue<'p>> {
        let entry = self.0.iter().find(|entry| entry.index == index)?;

        Some(&entry.value)
    }

    /// The value at `index`, if this update carries it, to edit in place.
    pub fn get_mut(&mut self, index: u8) -> Option<&mut MetadataValue<'p>> {
        let entry = self.0.iter_mut().find(|entry| entry.index == index)?;

        Some(&mut entry.value)
    }
}

impl<'p> PacketWriter<'p> for EntityMetadata<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let mut entries = Vec::new();

        loop {
            let index = u8::read(input)?;
            if index == END_OF_ENTRIES {
                break;
            }

            let kind = VarInt::read(input)?.0;
            let value = MetadataValue::read_value(kind, input)?;

            entries.push(MetadataEntry { index, value });
        }

        Ok(Self(entries))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        for entry in &self.0 {
            entry.index.encode(out)?;
            VarInt(entry.value.id()).encode(out)?;
            entry.value.encode_value(out)?;
        }

        END_OF_ENTRIES.encode(out)
    }

    fn body_len(&self) -> usize {
        let mut length = 1;

        for entry in &self.0 {
            length += 1 + VarInt(entry.value.id()).body_len() + entry.value.value_len();
        }

        length
    }
}

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct MetadataEntry<'p> {
    /// The index of the synced value, which depends on the entity's class.
    pub index: u8,
    pub value: MetadataValue<'p>,
}

tagged_enum! {
    /// A metadata value, sent after its serializer's [`metadata_type`] ID.
    /// Variants of a vanilla enum or registry hold its ID.
    pub enum MetadataValue<'p>, ids in metadata_type {
        /// Usually a set of bit flags.
        0 BYTE => Byte(u8),
        1 INT => Int(VarInt),
        2 LONG => Long(VarLong),
        3 FLOAT => Float(f32),
        4 STRING => String(Cow<'p, str>),
        5 TEXT => Text(Nbt<'p>),
        6 OPTIONAL_TEXT => OptionalText(Option<Nbt<'p>>),
        7 ITEM_STACK => ItemStack(Slot<'p>),
        8 BOOLEAN => Boolean(bool),
        /// Degrees around each axis, as of an armor stand's limbs.
        9 ROTATIONS => Rotations(Vector3f),
        /// A packed block position.
        10 BLOCK_POS => BlockPos(i64),
        11 OPTIONAL_BLOCK_POS => OptionalBlockPos(Option<i64>),
        /// A `Direction` ID: down, up, north, south, west, east.
        12 DIRECTION => Direction(VarInt),
        13 OPTIONAL_LIVING_ENTITY => OptionalLivingEntity(Option<Uuid>),
        14 BLOCK_STATE => BlockState(VarInt),
        /// A block state ID; 0, air, means absent.
        15 OPTIONAL_BLOCK_STATE => OptionalBlockState(VarInt),
        16 PARTICLE => Particle(Particle<'p>),
        17 PARTICLES => Particles(Vec<Particle<'p>>),
        18 VILLAGER_DATA => VillagerData(VillagerData),
        /// 0 means absent; any other value is the number plus one.
        19 OPTIONAL_UNSIGNED_INT => OptionalUnsignedInt(VarInt),
        20 POSE => Pose(VarInt),
        21 CAT_VARIANT => CatVariant(VarInt),
        22 CAT_SOUND_VARIANT => CatSoundVariant(VarInt),
        23 COW_VARIANT => CowVariant(VarInt),
        24 COW_SOUND_VARIANT => CowSoundVariant(VarInt),
        25 WOLF_VARIANT => WolfVariant(VarInt),
        26 WOLF_SOUND_VARIANT => WolfSoundVariant(VarInt),
        27 FROG_VARIANT => FrogVariant(VarInt),
        28 PIG_VARIANT => PigVariant(VarInt),
        29 PIG_SOUND_VARIANT => PigSoundVariant(VarInt),
        30 CHICKEN_VARIANT => ChickenVariant(VarInt),
        31 CHICKEN_SOUND_VARIANT => ChickenSoundVariant(VarInt),
        32 ZOMBIE_NAUTILUS_VARIANT => ZombieNautilusVariant(VarInt),
        33 OPTIONAL_GLOBAL_POS => OptionalGlobalPos(Option<GlobalPos<'p>>),
        34 PAINTING_VARIANT => PaintingVariant(IdOr<PaintingVariant<'p>>),
        35 SNIFFER_STATE => SnifferState(VarInt),
        36 ARMADILLO_STATE => ArmadilloState(VarInt),
        37 COPPER_GOLEM_STATE => CopperGolemState(VarInt),
        38 WEATHERING_COPPER_STATE => WeatheringCopperState(VarInt),
        39 VECTOR3 => Vector3(Vector3f),
        40 QUATERNION => Quaternion(Quaternion),
        41 RESOLVABLE_PROFILE => ResolvableProfile(ResolvableProfile<'p>),
        42 HUMANOID_ARM => HumanoidArm(VarInt),
        43 DYE_COLOR => DyeColor(VarInt),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PacketWriter, IntoOwned)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PacketWriter, IntoOwned)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct VillagerData {
    /// A `minecraft:villager_type` ID.
    #[format = "varint"]
    pub kind: i32,
    /// A `minecraft:villager_profession` ID.
    #[format = "varint"]
    pub profession: i32,
    #[format = "varint"]
    pub level: i32,
}

/// A painting defined inline rather than by its `minecraft:painting_variant` ID.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PaintingVariant<'p> {
    /// In blocks.
    #[format = "varint"]
    pub width: i32,
    #[format = "varint"]
    pub height: i32,
    pub asset_id: Cow<'p, str>,
    pub title: Option<Nbt<'p>>,
    pub author: Option<Nbt<'p>>,
}

#[cfg(test)]
#[path = "../../tests/tests_types_entity_metadata.rs"]
mod tests;
