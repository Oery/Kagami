//! The data component table of protocol 777, in `minecraft:data_component_type`
//! registration order: an entry's position is its wire ID.
use std::borrow::Cow;

use super::ItemStackTemplate;
use super::components::*;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::entity_metadata::PaintingVariant;
use crate::types::tagged::tagged_enum;
use crate::types::{HolderSet, IdOr, Nbt, ResolvableProfile, SoundEvent, VarInt};

tagged_enum! {
    /// A data component and its value. Variants of a vanilla enum or
    /// registry hold its ID; text is network NBT; identifiers are strings.
    pub enum Component<'p>, ids in component_type {
        0 CUSTOM_DATA => CustomData(Nbt<'p>),
        1 MAX_STACK_SIZE => MaxStackSize(VarInt),
        2 MAX_DAMAGE => MaxDamage(VarInt),
        3 DAMAGE => Damage(VarInt),
        4 UNBREAKABLE => Unbreakable,
        5 USE_EFFECTS => UseEffects(UseEffects),
        6 CUSTOM_NAME => CustomName(Nbt<'p>),
        7 MINIMUM_ATTACK_CHARGE => MinimumAttackCharge(f32),
        /// A `minecraft:damage_type` ID.
        8 DAMAGE_TYPE => DamageType(VarInt),
        9 ITEM_NAME => ItemName(Nbt<'p>),
        10 ITEM_MODEL => ItemModel(Cow<'p, str>),
        11 LORE => Lore(Vec<Nbt<'p>>),
        12 RARITY => Rarity(VarInt),
        13 ENCHANTMENTS => Enchantments(Vec<Enchantment>),
        14 CAN_PLACE_ON => CanPlaceOn(Vec<BlockPredicate<'p>>),
        15 CAN_BREAK => CanBreak(Vec<BlockPredicate<'p>>),
        16 ATTRIBUTE_MODIFIERS => AttributeModifiers(Vec<AttributeModifier<'p>>),
        17 CUSTOM_MODEL_DATA => CustomModelData(CustomModelData<'p>),
        18 TOOLTIP_DISPLAY => TooltipDisplay(TooltipDisplay),
        19 REPAIR_COST => RepairCost(VarInt),
        20 CREATIVE_SLOT_LOCK => CreativeSlotLock,
        21 ENCHANTMENT_GLINT_OVERRIDE => EnchantmentGlintOverride(bool),
        22 INTANGIBLE_PROJECTILE => IntangibleProjectile(Nbt<'p>),
        23 FOOD => Food(Food),
        24 CONSUMABLE => Consumable(Consumable<'p>),
        25 USE_REMAINDER => UseRemainder(ItemStackTemplate<'p>),
        26 USE_COOLDOWN => UseCooldown(UseCooldown<'p>),
        27 DAMAGE_RESISTANT => DamageResistant(HolderSet<'p>),
        28 TOOL => Tool(Tool<'p>),
        29 WEAPON => Weapon(Weapon),
        30 ATTACK_RANGE => AttackRange(AttackRange),
        31 ENCHANTABLE => Enchantable(VarInt),
        32 EQUIPPABLE => Equippable(Equippable<'p>),
        33 REPAIRABLE => Repairable(HolderSet<'p>),
        34 GLIDER => Glider,
        35 TOOLTIP_STYLE => TooltipStyle(Cow<'p, str>),
        36 DEATH_PROTECTION => DeathProtection(Vec<ConsumeEffect<'p>>),
        37 BLOCKS_ATTACKS => BlocksAttacks(BlocksAttacks<'p>),
        38 PIERCING_WEAPON => PiercingWeapon(PiercingWeapon<'p>),
        39 KINETIC_WEAPON => KineticWeapon(KineticWeapon<'p>),
        40 ATTACK_ANIMATION => AttackAnimation(SwingAnimation),
        41 INTERACT_ANIMATION => InteractAnimation(SwingAnimation),
        42 ADDITIONAL_TRADE_COST => AdditionalTradeCost(VarInt),
        43 BLOCK_TRANSFORMER => BlockTransformer(VarInt),
        /// The nutrition a villager gets from the item.
        44 VILLAGER_FOOD => VillagerFood(VarInt),
        45 STORED_ENCHANTMENTS => StoredEnchantments(Vec<Enchantment>),
        46 DYE => Dye(VarInt),
        /// An RGB color.
        47 DYED_COLOR => DyedColor(i32),
        48 MAP_ID => MapId(VarInt),
        49 MAP_DECORATIONS => MapDecorations(Nbt<'p>),
        50 MAP_POST_PROCESSING => MapPostProcessing(VarInt),
        51 CHARGED_PROJECTILES => ChargedProjectiles(Vec<ItemStackTemplate<'p>>),
        52 BUNDLE_CONTENTS => BundleContents(Vec<ItemStackTemplate<'p>>),
        53 POTION_CONTENTS => PotionContents(PotionContents<'p>),
        54 POTION_DURATION_SCALE => PotionDurationScale(f32),
        55 SUSPICIOUS_STEW_EFFECTS => SuspiciousStewEffects(Vec<StewEffect>),
        56 WRITABLE_BOOK_CONTENT => WritableBookContent(Vec<Filterable<Cow<'p, str>>>),
        57 WRITTEN_BOOK_CONTENT => WrittenBookContent(WrittenBookContent<'p>),
        58 TRIM => Trim(ArmorTrim<'p>),
        59 DEBUG_STICK_STATE => DebugStickState(Nbt<'p>),
        60 ENTITY_DATA => EntityData(TypedEntityData<'p>),
        61 BUCKET_ENTITY_DATA => BucketEntityData(Nbt<'p>),
        62 BLOCK_ENTITY_DATA => BlockEntityData(TypedEntityData<'p>),
        63 INSTRUMENT => Instrument(IdOr<Instrument<'p>>),
        64 PROVIDES_TRIM_MATERIAL => ProvidesTrimMaterial(IdOr<TrimMaterial<'p>>),
        65 OMINOUS_BOTTLE_AMPLIFIER => OminousBottleAmplifier(VarInt),
        66 JUKEBOX_PLAYABLE => JukeboxPlayable(IdOr<JukeboxSong<'p>>),
        67 PROVIDES_BANNER_PATTERNS => ProvidesBannerPatterns(HolderSet<'p>),
        68 RECIPES => Recipes(Nbt<'p>),
        69 LODESTONE_TRACKER => LodestoneTracker(LodestoneTracker<'p>),
        70 FIREWORK_EXPLOSION => FireworkExplosion(FireworkExplosion),
        71 FIREWORKS => Fireworks(Fireworks),
        72 PROFILE => Profile(ResolvableProfile<'p>),
        73 NOTE_BLOCK_SOUND => NoteBlockSound(Cow<'p, str>),
        74 BANNER_PATTERNS => BannerPatterns(Vec<BannerLayer<'p>>),
        75 BASE_COLOR => BaseColor(VarInt),
        76 POT_DECORATIONS => PotDecorations(PotDecorations<'p>),
        77 CONTAINER => Container(Vec<Option<ItemStackTemplate<'p>>>),
        78 BLOCK_STATE => BlockState(Vec<BlockStateProperty<'p>>),
        79 BEES => Bees(Vec<BeeOccupant<'p>>),
        80 SULFUR_CUBE_CONTENT => SulfurCubeContent(ItemStackTemplate<'p>),
        81 LOCK => Lock(Nbt<'p>),
        82 CONTAINER_LOOT => ContainerLoot(Nbt<'p>),
        83 BREAK_SOUND => BreakSound(IdOr<SoundEvent<'p>>),
        /// The composter layers the item adds.
        84 COMPOSTABLE => Compostable(ResolvableInt<'p>),
        85 COOKING_FUEL => CookingFuel(Fuel<'p>),
        86 BREWING_FUEL => BrewingFuel(Fuel<'p>),
        87 MOB_VISIBILITY => MobVisibility(MobVisibility<'p>),
        88 VILLAGER_VARIANT => VillagerVariant(VarInt),
        89 WOLF_VARIANT => WolfVariant(VarInt),
        90 WOLF_SOUND_VARIANT => WolfSoundVariant(VarInt),
        91 WOLF_COLLAR => WolfCollar(VarInt),
        92 FOX_VARIANT => FoxVariant(VarInt),
        93 SALMON_SIZE => SalmonSize(VarInt),
        94 PARROT_VARIANT => ParrotVariant(VarInt),
        95 TROPICAL_FISH_PATTERN => TropicalFishPattern(VarInt),
        96 TROPICAL_FISH_BASE_COLOR => TropicalFishBaseColor(VarInt),
        97 TROPICAL_FISH_PATTERN_COLOR => TropicalFishPatternColor(VarInt),
        98 MOOSHROOM_VARIANT => MooshroomVariant(VarInt),
        99 RABBIT_VARIANT => RabbitVariant(VarInt),
        100 PIG_VARIANT => PigVariant(VarInt),
        101 PIG_SOUND_VARIANT => PigSoundVariant(VarInt),
        102 COW_VARIANT => CowVariant(VarInt),
        103 COW_SOUND_VARIANT => CowSoundVariant(VarInt),
        104 CHICKEN_VARIANT => ChickenVariant(VarInt),
        105 CHICKEN_SOUND_VARIANT => ChickenSoundVariant(VarInt),
        106 ZOMBIE_NAUTILUS_VARIANT => ZombieNautilusVariant(VarInt),
        107 FROG_VARIANT => FrogVariant(VarInt),
        108 HORSE_VARIANT => HorseVariant(VarInt),
        109 PAINTING_VARIANT => PaintingVariant(IdOr<PaintingVariant<'p>>),
        110 LLAMA_VARIANT => LlamaVariant(VarInt),
        111 AXOLOTL_VARIANT => AxolotlVariant(VarInt),
        112 CAT_VARIANT => CatVariant(VarInt),
        113 CAT_SOUND_VARIANT => CatSoundVariant(VarInt),
        114 CAT_COLLAR => CatCollar(VarInt),
        115 SHEEP_COLOR => SheepColor(VarInt),
        116 SHULKER_COLOR => ShulkerColor(VarInt),
        117 PROVIDES_POTTERY_PATTERN => ProvidesPotteryPattern(VarInt),
        118 SIGN_TEXT_FRONT => SignTextFront(SignText<'p>),
        119 SIGN_TEXT_BACK => SignTextBack(SignText<'p>),
        120 WAXED => Waxed,
        121 CUSHION_COLOR => CushionColor(VarInt),
    }
}

/// A component with its ID first, as in exact item predicates.
impl<'p> PacketWriter<'p> for Component<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let id = VarInt::read(input)?.0;

        Self::read_value(id, input)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.id()).encode(out)?;

        self.encode_value(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.id()).body_len() + self.value_len()
    }
}
