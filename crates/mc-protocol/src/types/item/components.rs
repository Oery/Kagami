//! Values of the data components in [`Component`](super::Component).
//!
//! Fields named after a vanilla enum or registry hold its ID. Text is network
//! NBT and identifiers are strings, as elsewhere in this crate.
use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use super::{Component, ItemStackTemplate};
use crate::IntoOwned;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Either, GlobalPos, HolderSet, IdOr, Nbt, SoundEvent, VarInt};

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct UseEffects {
    pub can_sprint: bool,
    pub interact_vibrations: bool,
    pub speed_multiplier: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Enchantment {
    /// A `minecraft:enchantment` ID.
    #[format = "varint"]
    pub id: i32,
    #[format = "varint"]
    pub level: i32,
}

/// A block an adventure-mode player may place on or break.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BlockPredicate<'p> {
    /// `minecraft:block` entries.
    pub blocks: Option<HolderSet<'p>>,
    pub properties: Option<Vec<PropertyMatcher<'p>>>,
    pub nbt: Option<Nbt<'p>>,
    /// Components the block entity must have with exactly these values.
    pub exact_components: Vec<Component<'p>>,
    pub partial_components: Vec<PartialComponentPredicate<'p>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PropertyMatcher<'p> {
    pub name: Cow<'p, str>,
    /// An exact value, or an inclusive range.
    pub value: Either<Cow<'p, str>, PropertyRange<'p>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PropertyRange<'p> {
    pub min: Option<Cow<'p, str>>,
    pub max: Option<Cow<'p, str>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PartialComponentPredicate<'p> {
    /// A `minecraft:data_component_predicate_type` ID, or a
    /// [`component_type`](super::component_type) that only has to be present.
    pub kind: Either<VarInt, VarInt>,
    /// The predicate's arguments; an empty compound for a presence check.
    pub value: Nbt<'p>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct AttributeModifier<'p> {
    /// A `minecraft:attribute` ID.
    #[format = "varint"]
    pub attribute: i32,
    /// The modifier's identifier.
    pub id: Cow<'p, str>,
    pub amount: f64,
    /// 0 adds the amount, 1 adds it times the base, 2 multiplies the total.
    #[format = "varint"]
    pub operation: i32,
    /// An `EquipmentSlotGroup` ID: 0 is any slot.
    #[format = "varint"]
    pub slot: i32,
    pub display: AttributeDisplay<'p>,
}

/// How the tooltip shows an attribute modifier, sent as an ID then its data.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum AttributeDisplay<'p> {
    Default,
    Hidden,
    Override(Nbt<'p>),
}

impl<'p> PacketWriter<'p> for AttributeDisplay<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let display = match VarInt::read(input)?.0 {
            0 => Self::Default,
            1 => Self::Hidden,
            2 => Self::Override(Nbt::read(input)?),
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        Ok(display)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Default => VarInt(0).encode(out),
            Self::Hidden => VarInt(1).encode(out),
            Self::Override(text) => {
                VarInt(2).encode(out)?;
                text.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::Default | Self::Hidden => 1,
            Self::Override(text) => 1 + text.body_len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct CustomModelData<'p> {
    pub floats: Vec<f32>,
    pub flags: Vec<bool>,
    pub strings: Vec<Cow<'p, str>>,
    /// RGB colors.
    pub colors: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    /// [`component_type`](super::component_type) IDs left out of the tooltip.
    pub hidden_components: Vec<VarInt>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Food {
    #[format = "varint"]
    pub nutrition: i32,
    pub saturation: f32,
    pub can_always_eat: bool,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Consumable<'p> {
    pub consume_seconds: f32,
    /// An `ItemUseAnimation` ID.
    #[format = "varint"]
    pub animation: i32,
    pub sound: IdOr<SoundEvent<'p>>,
    pub has_consume_particles: bool,
    pub on_consume_effects: Vec<ConsumeEffect<'p>>,
}

/// An effect of consuming an item, sent as its `minecraft:consume_effect_type`
/// ID then its data.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum ConsumeEffect<'p> {
    ApplyEffects {
        effects: Vec<MobEffectInstance>,
        probability: f32,
    },
    /// `minecraft:mob_effect` entries.
    RemoveEffects(HolderSet<'p>),
    ClearAllEffects,
    TeleportRandomly {
        diameter: f32,
        directional_particles: bool,
    },
    PlaySound(IdOr<SoundEvent<'p>>),
}

impl<'p> PacketWriter<'p> for ConsumeEffect<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let effect = match VarInt::read(input)?.0 {
            0 => {
                let effects = Vec::read(input)?;
                let probability = f32::read(input)?;

                Self::ApplyEffects { effects, probability }
            }
            1 => Self::RemoveEffects(HolderSet::read(input)?),
            2 => Self::ClearAllEffects,
            3 => {
                let diameter = f32::read(input)?;
                let directional_particles = bool::read(input)?;

                Self::TeleportRandomly { diameter, directional_particles }
            }
            4 => Self::PlaySound(IdOr::read(input)?),
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        Ok(effect)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.id()).encode(out)?;

        match self {
            Self::ApplyEffects { effects, probability } => {
                effects.encode(out)?;
                probability.encode(out)
            }
            Self::RemoveEffects(effects) => effects.encode(out),
            Self::ClearAllEffects => Ok(()),
            Self::TeleportRandomly { diameter, directional_particles } => {
                diameter.encode(out)?;
                directional_particles.encode(out)
            }
            Self::PlaySound(sound) => sound.encode(out),
        }
    }

    fn body_len(&self) -> usize {
        let data = match self {
            Self::ApplyEffects { effects, .. } => effects.body_len() + 4,
            Self::RemoveEffects(effects) => effects.body_len(),
            Self::ClearAllEffects => 0,
            Self::TeleportRandomly { .. } => 4 + 1,
            Self::PlaySound(sound) => sound.body_len(),
        };

        1 + data
    }
}

impl ConsumeEffect<'_> {
    fn id(&self) -> i32 {
        match self {
            Self::ApplyEffects { .. } => 0,
            Self::RemoveEffects(_) => 1,
            Self::ClearAllEffects => 2,
            Self::TeleportRandomly { .. } => 3,
            Self::PlaySound(_) => 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct MobEffectInstance {
    /// A `minecraft:mob_effect` ID.
    #[format = "varint"]
    pub effect: i32,
    pub details: MobEffectDetails,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct MobEffectDetails {
    #[format = "varint"]
    pub amplifier: i32,
    /// In ticks; -1 is infinite.
    #[format = "varint"]
    pub duration: i32,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    /// A weaker instance of the same effect that resumes when this one ends.
    pub hidden_effect: Option<Box<MobEffectDetails>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct UseCooldown<'p> {
    pub seconds: f32,
    /// Items sharing a group share a cooldown; the item's own ID otherwise.
    pub group: Option<Cow<'p, str>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Tool<'p> {
    pub rules: Vec<ToolRule<'p>>,
    pub default_mining_speed: f32,
    #[format = "varint"]
    pub damage_per_block: i32,
    pub can_destroy_blocks_in_creative: bool,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct ToolRule<'p> {
    /// `minecraft:block` entries.
    pub blocks: HolderSet<'p>,
    pub speed: Option<f32>,
    pub correct_for_drops: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Weapon {
    #[format = "varint"]
    pub item_damage_per_attack: i32,
    pub disable_blocking_for_seconds: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Equippable<'p> {
    /// An `EquipmentSlot` ID.
    #[format = "varint"]
    pub slot: i32,
    pub equip_sound: IdOr<SoundEvent<'p>>,
    pub asset_id: Option<Cow<'p, str>>,
    pub camera_overlay: Option<Cow<'p, str>>,
    /// `minecraft:entity_type` entries.
    pub allowed_entities: Option<HolderSet<'p>>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
    pub equip_on_interact: bool,
    pub can_be_sheared: bool,
    pub shearing_sound: IdOr<SoundEvent<'p>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BlocksAttacks<'p> {
    pub block_delay_seconds: f32,
    pub disable_cooldown_scale: f32,
    pub damage_reductions: Vec<DamageReduction<'p>>,
    pub item_damage: ItemDamageFunction,
    /// `minecraft:damage_type` entries.
    pub bypassed_by: Option<HolderSet<'p>>,
    pub block_sound: Option<IdOr<SoundEvent<'p>>>,
    pub disable_sound: Option<IdOr<SoundEvent<'p>>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct DamageReduction<'p> {
    pub horizontal_blocking_angle: f32,
    /// `minecraft:damage_type` entries; every type when absent.
    pub kind: Option<HolderSet<'p>>,
    pub base: f32,
    pub factor: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct ItemDamageFunction {
    pub threshold: f32,
    pub base: f32,
    pub factor: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PiercingWeapon<'p> {
    pub deals_knockback: bool,
    pub dismounts: bool,
    pub sound: Option<IdOr<SoundEvent<'p>>>,
    pub hit_sound: Option<IdOr<SoundEvent<'p>>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct KineticWeapon<'p> {
    #[format = "varint"]
    pub contact_cooldown_ticks: i32,
    #[format = "varint"]
    pub delay_ticks: i32,
    pub dismount_conditions: Option<KineticCondition>,
    pub knockback_conditions: Option<KineticCondition>,
    pub damage_conditions: Option<KineticCondition>,
    pub forward_movement: f32,
    pub damage_multiplier: f32,
    pub sound: Option<IdOr<SoundEvent<'p>>>,
    pub hit_sound: Option<IdOr<SoundEvent<'p>>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct KineticCondition {
    #[format = "varint"]
    pub max_duration_ticks: i32,
    pub min_speed: f32,
    pub min_relative_speed: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct SwingAnimation {
    /// A `SwingAnimationType` ID.
    #[format = "varint"]
    pub kind: i32,
    #[format = "varint"]
    pub duration: i32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PotionContents<'p> {
    /// A `minecraft:potion` ID.
    pub potion: Option<VarInt>,
    /// An RGB color.
    pub custom_color: Option<i32>,
    pub custom_effects: Vec<MobEffectInstance>,
    pub custom_name: Option<Cow<'p, str>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct StewEffect {
    /// A `minecraft:mob_effect` ID.
    #[format = "varint"]
    pub effect: i32,
    #[format = "varint"]
    pub duration: i32,
}

/// A value and, when chat filtering changed it, its filtered form.
#[derive(Clone, Debug, PartialEq)]
pub struct Filterable<T> {
    pub raw: T,
    pub filtered: Option<T>,
}

impl<'p, T: PacketWriter<'p>> PacketWriter<'p> for Filterable<T> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let raw = T::read(input)?;
        let filtered = Option::read(input)?;

        Ok(Self { raw, filtered })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.raw.encode(out)?;

        self.filtered.encode(out)
    }

    fn body_len(&self) -> usize {
        self.raw.body_len() + self.filtered.body_len()
    }
}

impl<T: IntoOwned> IntoOwned for Filterable<T> {
    type Owned = Filterable<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        Filterable { raw: self.raw.into_owned(), filtered: self.filtered.into_owned() }
    }
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct WrittenBookContent<'p> {
    pub title: Filterable<Cow<'p, str>>,
    pub author: Cow<'p, str>,
    /// 0 is the original; each copy adds one, up to 3.
    #[format = "varint"]
    pub generation: i32,
    pub pages: Vec<Filterable<Nbt<'p>>>,
    pub resolved: bool,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct ArmorTrim<'p> {
    pub material: IdOr<TrimMaterial<'p>>,
    pub pattern: IdOr<TrimPattern<'p>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct TrimMaterial<'p> {
    pub palette_id: Cow<'p, str>,
    pub description: Nbt<'p>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct TrimPattern<'p> {
    pub asset_id: Cow<'p, str>,
    pub description: Nbt<'p>,
    pub decal: bool,
}

/// An entity or block entity's saved data, with the type it belongs to.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct TypedEntityData<'p> {
    /// A `minecraft:entity_type` or `minecraft:block_entity_type` ID.
    #[format = "varint"]
    pub kind: i32,
    pub data: Nbt<'p>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Instrument<'p> {
    pub sound: IdOr<SoundEvent<'p>>,
    pub use_duration: f32,
    pub range: f32,
    #[format = "varint"]
    pub durability_damage: i32,
    pub description: Nbt<'p>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct JukeboxSong<'p> {
    pub sound: IdOr<SoundEvent<'p>>,
    pub description: Nbt<'p>,
    pub length_in_seconds: f32,
    #[format = "varint"]
    pub comparator_output: i32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct LodestoneTracker<'p> {
    pub target: Option<GlobalPos<'p>>,
    pub tracked: bool,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct FireworkExplosion {
    /// A `FireworkExplosion.Shape` ID.
    #[format = "varint"]
    pub shape: i32,
    /// RGB colors.
    pub colors: Vec<i32>,
    pub fade_colors: Vec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Fireworks {
    #[format = "varint"]
    pub flight_duration: i32,
    pub explosions: Vec<FireworkExplosion>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BannerLayer<'p> {
    pub pattern: IdOr<BannerPattern<'p>>,
    /// A `DyeColor` ID.
    #[format = "varint"]
    pub color: i32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BannerPattern<'p> {
    pub asset_id: Cow<'p, str>,
    pub translation_key: Cow<'p, str>,
}

/// The sherds or bricks on each side of a decorated pot.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PotDecorations<'p> {
    pub back: Option<ItemStackTemplate<'p>>,
    pub left: Option<ItemStackTemplate<'p>>,
    pub right: Option<ItemStackTemplate<'p>>,
    pub front: Option<ItemStackTemplate<'p>>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BlockStateProperty<'p> {
    pub name: Cow<'p, str>,
    pub value: Cow<'p, str>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct BeeOccupant<'p> {
    pub entity_data: TypedEntityData<'p>,
    #[format = "varint"]
    pub ticks_in_hive: i32,
    #[format = "varint"]
    pub min_ticks_in_hive: i32,
}

/// A constant, or a `minecraft:context_int_provider` identifier.
pub type ResolvableInt<'p> = Either<i32, Cow<'p, str>>;

/// A constant, or a `minecraft:context_float_provider` identifier.
pub type ResolvableFloat<'p> = Either<f32, Cow<'p, str>>;

/// A cooking or brewing fuel: burn time or uses, and a speed multiplier.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct Fuel<'p> {
    pub amount: ResolvableInt<'p>,
    pub speed_multiplier: ResolvableFloat<'p>,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct MobVisibility<'p> {
    /// `minecraft:entity_type` entries.
    pub targeting_entity_types: HolderSet<'p>,
    pub visibility: f32,
}

#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct SignText<'p> {
    pub messages: [Nbt<'p>; 4],
    pub filtered_messages: Option<[Nbt<'p>; 4]>,
    /// A `DyeColor` ID.
    #[format = "varint"]
    pub color: i32,
    pub has_glowing_text: bool,
}
