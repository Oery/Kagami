//! Vanilla Config registry data for 26.3, generated from client.jar JSON.
//! Entry order defines numeric IDs from 0. See gen_registries.py (dev machine only).

use crate::types::Nbt;

type Entries<'a> = Vec<(&'a str, Nbt<'a>)>;
type Tags = Vec<(&'static str, Vec<i32>)>;

pub fn banner_pattern() -> Entries<'static> {
    vec![
        (
            "minecraft:base",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:base")),
                ("translation_key", Nbt::Str("block.minecraft.banner.base")),
            ]),
        ),
        (
            "minecraft:border",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:border")),
                ("translation_key", Nbt::Str("block.minecraft.banner.border")),
            ]),
        ),
        (
            "minecraft:bricks",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:bricks")),
                ("translation_key", Nbt::Str("block.minecraft.banner.bricks")),
            ]),
        ),
        (
            "minecraft:circle",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:circle")),
                ("translation_key", Nbt::Str("block.minecraft.banner.circle")),
            ]),
        ),
        (
            "minecraft:creeper",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:creeper")),
                ("translation_key", Nbt::Str("block.minecraft.banner.creeper")),
            ]),
        ),
        (
            "minecraft:cross",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:cross")),
                ("translation_key", Nbt::Str("block.minecraft.banner.cross")),
            ]),
        ),
        (
            "minecraft:curly_border",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:curly_border")),
                ("translation_key", Nbt::Str("block.minecraft.banner.curly_border")),
            ]),
        ),
        (
            "minecraft:diagonal_left",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:diagonal_left")),
                ("translation_key", Nbt::Str("block.minecraft.banner.diagonal_left")),
            ]),
        ),
        (
            "minecraft:diagonal_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:diagonal_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.diagonal_right")),
            ]),
        ),
        (
            "minecraft:diagonal_up_left",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:diagonal_up_left")),
                ("translation_key", Nbt::Str("block.minecraft.banner.diagonal_up_left")),
            ]),
        ),
        (
            "minecraft:diagonal_up_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:diagonal_up_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.diagonal_up_right")),
            ]),
        ),
        (
            "minecraft:flow",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:flow")),
                ("translation_key", Nbt::Str("block.minecraft.banner.flow")),
            ]),
        ),
        (
            "minecraft:flower",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:flower")),
                ("translation_key", Nbt::Str("block.minecraft.banner.flower")),
            ]),
        ),
        (
            "minecraft:globe",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:globe")),
                ("translation_key", Nbt::Str("block.minecraft.banner.globe")),
            ]),
        ),
        (
            "minecraft:gradient",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:gradient")),
                ("translation_key", Nbt::Str("block.minecraft.banner.gradient")),
            ]),
        ),
        (
            "minecraft:gradient_up",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:gradient_up")),
                ("translation_key", Nbt::Str("block.minecraft.banner.gradient_up")),
            ]),
        ),
        (
            "minecraft:guster",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:guster")),
                ("translation_key", Nbt::Str("block.minecraft.banner.guster")),
            ]),
        ),
        (
            "minecraft:half_horizontal",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:half_horizontal")),
                ("translation_key", Nbt::Str("block.minecraft.banner.half_horizontal")),
            ]),
        ),
        (
            "minecraft:half_horizontal_bottom",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:half_horizontal_bottom")),
                ("translation_key", Nbt::Str("block.minecraft.banner.half_horizontal_bottom")),
            ]),
        ),
        (
            "minecraft:half_vertical",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:half_vertical")),
                ("translation_key", Nbt::Str("block.minecraft.banner.half_vertical")),
            ]),
        ),
        (
            "minecraft:half_vertical_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:half_vertical_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.half_vertical_right")),
            ]),
        ),
        (
            "minecraft:mojang",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:mojang")),
                ("translation_key", Nbt::Str("block.minecraft.banner.mojang")),
            ]),
        ),
        (
            "minecraft:piglin",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:piglin")),
                ("translation_key", Nbt::Str("block.minecraft.banner.piglin")),
            ]),
        ),
        (
            "minecraft:rhombus",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:rhombus")),
                ("translation_key", Nbt::Str("block.minecraft.banner.rhombus")),
            ]),
        ),
        (
            "minecraft:skull",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:skull")),
                ("translation_key", Nbt::Str("block.minecraft.banner.skull")),
            ]),
        ),
        (
            "minecraft:small_stripes",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:small_stripes")),
                ("translation_key", Nbt::Str("block.minecraft.banner.small_stripes")),
            ]),
        ),
        (
            "minecraft:square_bottom_left",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:square_bottom_left")),
                ("translation_key", Nbt::Str("block.minecraft.banner.square_bottom_left")),
            ]),
        ),
        (
            "minecraft:square_bottom_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:square_bottom_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.square_bottom_right")),
            ]),
        ),
        (
            "minecraft:square_top_left",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:square_top_left")),
                ("translation_key", Nbt::Str("block.minecraft.banner.square_top_left")),
            ]),
        ),
        (
            "minecraft:square_top_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:square_top_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.square_top_right")),
            ]),
        ),
        (
            "minecraft:straight_cross",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:straight_cross")),
                ("translation_key", Nbt::Str("block.minecraft.banner.straight_cross")),
            ]),
        ),
        (
            "minecraft:stripe_bottom",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_bottom")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_bottom")),
            ]),
        ),
        (
            "minecraft:stripe_center",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_center")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_center")),
            ]),
        ),
        (
            "minecraft:stripe_downleft",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_downleft")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_downleft")),
            ]),
        ),
        (
            "minecraft:stripe_downright",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_downright")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_downright")),
            ]),
        ),
        (
            "minecraft:stripe_left",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_left")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_left")),
            ]),
        ),
        (
            "minecraft:stripe_middle",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_middle")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_middle")),
            ]),
        ),
        (
            "minecraft:stripe_right",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_right")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_right")),
            ]),
        ),
        (
            "minecraft:stripe_top",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stripe_top")),
                ("translation_key", Nbt::Str("block.minecraft.banner.stripe_top")),
            ]),
        ),
        (
            "minecraft:triangle_bottom",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:triangle_bottom")),
                ("translation_key", Nbt::Str("block.minecraft.banner.triangle_bottom")),
            ]),
        ),
        (
            "minecraft:triangle_top",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:triangle_top")),
                ("translation_key", Nbt::Str("block.minecraft.banner.triangle_top")),
            ]),
        ),
        (
            "minecraft:triangles_bottom",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:triangles_bottom")),
                ("translation_key", Nbt::Str("block.minecraft.banner.triangles_bottom")),
            ]),
        ),
        (
            "minecraft:triangles_top",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:triangles_top")),
                ("translation_key", Nbt::Str("block.minecraft.banner.triangles_top")),
            ]),
        ),
    ]
}

pub fn block_transformer() -> Entries<'static> {
    vec![
        (
            "minecraft:axe",
            Nbt::List(vec![
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oak_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_oak_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oak_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_oak_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:dark_oak_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_dark_oak_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:dark_oak_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_dark_oak_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:pale_oak_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_pale_oak_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:pale_oak_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_pale_oak_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:poplar_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_poplar_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:poplar_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_poplar_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:acacia_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_acacia_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:acacia_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_acacia_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:cherry_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_cherry_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:cherry_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_cherry_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:birch_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_birch_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:birch_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_birch_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:jungle_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_jungle_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:jungle_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_jungle_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:spruce_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_spruce_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:spruce_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_spruce_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:warped_stem")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_warped_stem")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:warped_hyphae")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_warped_hyphae")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:crimson_stem")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_crimson_stem")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:crimson_hyphae")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_crimson_hyphae")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:mangrove_wood")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_mangrove_wood")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:mangrove_log")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_mangrove_log")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:bamboo_block")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:stripped_bamboo_block")),
                                                        ("properties", Nbt::Compound(vec![("axis", Nbt::Str("y"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("sound", Nbt::Str("minecraft:item.axe.strip")),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:copper_block"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("scrape")),
                    ("sound", Nbt::Str("minecraft:item.axe.scrape")),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("scrape")),
                    ("sound", Nbt::Str("minecraft:item.axe.scrape")),
                    ("transform_type", Nbt::Str("copper_chest")),
                    ("update_from_neighbors", Nbt::Bool(false)),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:exposed_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:weathered_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:oxidized_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("scrape")),
                    ("sound", Nbt::Str("minecraft:item.axe.scrape")),
                    ("update_from_neighbors", Nbt::Bool(false)),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_block")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:copper_block"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:oxidized_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_cut_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:oxidized_cut_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_cut_copper_slab")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_cut_copper_slab")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("bottom")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_cut_copper_stairs")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_cut_copper_stairs")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("shape", Nbt::Str("straight")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:exposed_chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:weathered_chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_chiseled_copper")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                ("source", Nbt::Compound(vec![("id", Nbt::Str("minecraft:oxidized_chiseled_copper"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_trapdoor")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_trapdoor")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("bottom")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_bars")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_bars")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("east", Nbt::Str("false")),
                                                                ("north", Nbt::Str("false")),
                                                                ("south", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                                ("west", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_grate")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_grate")),
                                                        ("properties", Nbt::Compound(vec![("waterlogged", Nbt::Str("false"))])),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_bulb")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_bulb")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![("lit", Nbt::Str("false")), ("powered", Nbt::Str("false"))]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_golem_statue")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_golem_statue")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("copper_golem_pose", Nbt::Str("standing")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_lightning_rod")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_lightning_rod")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("up")),
                                                                ("powered", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_lantern")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_lantern")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("hanging", Nbt::Str("false")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_chain")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_chain")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("axis", Nbt::Str("y")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("wax_off")),
                    ("sound", Nbt::Str("minecraft:item.axe.wax_off")),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_chest")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_chest")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("single")),
                                                                ("facing", Nbt::Str("north")),
                                                                ("waterlogged", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("wax_off")),
                    ("sound", Nbt::Str("minecraft:item.axe.wax_off")),
                    ("transform_type", Nbt::Str("copper_chest")),
                    ("update_from_neighbors", Nbt::Bool(false)),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_exposed_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:exposed_copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_weathered_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:weathered_copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:matching_blocks")),
                                                ("blocks", Nbt::Str("minecraft:waxed_oxidized_copper_door")),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:copy_properties")),
                                                (
                                                    "source",
                                                    Nbt::Compound(vec![
                                                        ("id", Nbt::Str("minecraft:oxidized_copper_door")),
                                                        (
                                                            "properties",
                                                            Nbt::Compound(vec![
                                                                ("facing", Nbt::Str("north")),
                                                                ("half", Nbt::Str("lower")),
                                                                ("hinge", Nbt::Str("left")),
                                                                ("open", Nbt::Str("false")),
                                                                ("powered", Nbt::Str("false")),
                                                            ]),
                                                        ),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("particle", Nbt::Str("wax_off")),
                    ("sound", Nbt::Str("minecraft:item.axe.wax_off")),
                    ("update_from_neighbors", Nbt::Bool(false)),
                ]),
            ]),
        ),
        (
            "minecraft:hoe",
            Nbt::List(vec![
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:all_of")),
                                                (
                                                    "predicates",
                                                    Nbt::List(vec![
                                                        Nbt::Compound(vec![
                                                            ("type", Nbt::Str("minecraft:matching_block_tag")),
                                                            ("tag", Nbt::Str("minecraft:turns_into_farmland")),
                                                        ]),
                                                        Nbt::Compound(vec![
                                                            ("type", Nbt::Str("minecraft:matching_block_tag")),
                                                            ("offset", Nbt::List(vec![Nbt::Int(0), Nbt::Int(1), Nbt::Int(0)])),
                                                            ("tag", Nbt::Str("minecraft:air")),
                                                        ]),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                        (
                                            "then",
                                            Nbt::Compound(vec![
                                                ("id", Nbt::Str("minecraft:farmland")),
                                                ("properties", Nbt::Compound(vec![("moisture", Nbt::Str("0"))])),
                                            ]),
                                        ),
                                    ]),
                                    Nbt::Compound(vec![
                                        (
                                            "if_true",
                                            Nbt::Compound(vec![
                                                ("type", Nbt::Str("minecraft:all_of")),
                                                (
                                                    "predicates",
                                                    Nbt::List(vec![
                                                        Nbt::Compound(vec![
                                                            ("type", Nbt::Str("minecraft:matching_blocks")),
                                                            ("blocks", Nbt::Str("minecraft:coarse_dirt")),
                                                        ]),
                                                        Nbt::Compound(vec![
                                                            ("type", Nbt::Str("minecraft:matching_block_tag")),
                                                            ("offset", Nbt::List(vec![Nbt::Int(0), Nbt::Int(1), Nbt::Int(0)])),
                                                            ("tag", Nbt::Str("minecraft:air")),
                                                        ]),
                                                    ]),
                                                ),
                                            ]),
                                        ),
                                        ("then", Nbt::Compound(vec![("id", Nbt::Str("minecraft:dirt"))])),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                    ("disallowed_faces", Nbt::List(vec![Nbt::Str("down")])),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("sound", Nbt::Str("minecraft:item.hoe.till")),
                ]),
                Nbt::Compound(vec![
                    (
                        "block_state_provider",
                        Nbt::Compound(vec![
                            ("type", Nbt::Str("minecraft:rule_based")),
                            (
                                "rules",
                                Nbt::List(vec![Nbt::Compound(vec![
                                    (
                                        "if_true",
                                        Nbt::Compound(vec![
                                            ("type", Nbt::Str("minecraft:matching_blocks")),
                                            ("blocks", Nbt::Str("minecraft:rooted_dirt")),
                                        ]),
                                    ),
                                    ("then", Nbt::Compound(vec![("id", Nbt::Str("minecraft:dirt"))])),
                                ])]),
                            ),
                        ]),
                    ),
                    ("drop_strategy", Nbt::Str("clicked_face")),
                    ("item_damage_per_use", Nbt::Int(1)),
                    ("loot", Nbt::Str("minecraft:till/rooted_dirt")),
                    ("sound", Nbt::Str("minecraft:item.hoe.till")),
                ]),
            ]),
        ),
        (
            "minecraft:shovel",
            Nbt::List(vec![Nbt::Compound(vec![
                (
                    "block_state_provider",
                    Nbt::Compound(vec![
                        ("type", Nbt::Str("minecraft:rule_based")),
                        (
                            "rules",
                            Nbt::List(vec![Nbt::Compound(vec![
                                (
                                    "if_true",
                                    Nbt::Compound(vec![
                                        ("type", Nbt::Str("minecraft:all_of")),
                                        (
                                            "predicates",
                                            Nbt::List(vec![
                                                Nbt::Compound(vec![
                                                    ("type", Nbt::Str("minecraft:matching_block_tag")),
                                                    ("tag", Nbt::Str("minecraft:turns_into_dirt_path")),
                                                ]),
                                                Nbt::Compound(vec![
                                                    ("type", Nbt::Str("minecraft:matching_block_tag")),
                                                    ("offset", Nbt::List(vec![Nbt::Int(0), Nbt::Int(1), Nbt::Int(0)])),
                                                    ("tag", Nbt::Str("minecraft:air")),
                                                ]),
                                            ]),
                                        ),
                                    ]),
                                ),
                                ("then", Nbt::Compound(vec![("id", Nbt::Str("minecraft:dirt_path"))])),
                            ])]),
                        ),
                    ]),
                ),
                ("disallowed_faces", Nbt::List(vec![Nbt::Str("down")])),
                ("item_damage_per_use", Nbt::Int(1)),
                ("sound", Nbt::Str("minecraft:item.shovel.flatten")),
            ])]),
        ),
    ]
}

pub fn cat_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:all_black",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_all_black")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_all_black_baby")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            (
                                "condition",
                                Nbt::Compound(vec![
                                    ("type", Nbt::Str("minecraft:structure")),
                                    ("structures", Nbt::Str("#minecraft:cats_spawn_as_black")),
                                ]),
                            ),
                            ("priority", Nbt::Int(1)),
                        ]),
                        Nbt::Compound(vec![
                            (
                                "condition",
                                Nbt::Compound(vec![
                                    ("type", Nbt::Str("minecraft:moon_brightness")),
                                    ("range", Nbt::Compound(vec![("min", Nbt::Float(0.9_f32))])),
                                ]),
                            ),
                            ("priority", Nbt::Int(0)),
                        ]),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:black",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_black")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_black_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:british_shorthair",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_british_shorthair")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_british_shorthair_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:calico",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_calico")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_calico_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:jellie",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_jellie")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_jellie_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:persian",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_persian")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_persian_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:ragdoll",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_ragdoll")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_ragdoll_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:red",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_red")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_red_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:siamese",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_siamese")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_siamese_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:tabby",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_tabby")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_tabby_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:white",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cat/cat_white")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cat/cat_white_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
    ]
}

pub fn cat_sound_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:classic",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.cat.ambient")),
                        ("beg_for_food_sound", Nbt::Str("minecraft:entity.cat.beg_for_food")),
                        ("death_sound", Nbt::Str("minecraft:entity.cat.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.cat.eat")),
                        ("hiss_sound", Nbt::Str("minecraft:entity.cat.hiss")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.cat.hurt")),
                        ("purr_sound", Nbt::Str("minecraft:entity.cat.purr")),
                        ("purreow_sound", Nbt::Str("minecraft:entity.cat.purreow")),
                        ("stray_ambient_sound", Nbt::Str("minecraft:entity.cat.stray_ambient")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_cat.ambient")),
                        ("beg_for_food_sound", Nbt::Str("minecraft:entity.baby_cat.beg_for_food")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_cat.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.baby_cat.eat")),
                        ("hiss_sound", Nbt::Str("minecraft:entity.baby_cat.hiss")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_cat.hurt")),
                        ("purr_sound", Nbt::Str("minecraft:entity.baby_cat.purr")),
                        ("purreow_sound", Nbt::Str("minecraft:entity.baby_cat.purreow")),
                        ("stray_ambient_sound", Nbt::Str("minecraft:entity.baby_cat.stray_ambient")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:royal",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.cat_royal.ambient")),
                        ("beg_for_food_sound", Nbt::Str("minecraft:entity.cat_royal.beg_for_food")),
                        ("death_sound", Nbt::Str("minecraft:entity.cat_royal.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.cat_royal.eat")),
                        ("hiss_sound", Nbt::Str("minecraft:entity.cat_royal.hiss")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.cat_royal.hurt")),
                        ("purr_sound", Nbt::Str("minecraft:entity.cat_royal.purr")),
                        ("purreow_sound", Nbt::Str("minecraft:entity.cat_royal.purreow")),
                        ("stray_ambient_sound", Nbt::Str("minecraft:entity.cat_royal.stray_ambient")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_cat.ambient")),
                        ("beg_for_food_sound", Nbt::Str("minecraft:entity.baby_cat.beg_for_food")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_cat.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.baby_cat.eat")),
                        ("hiss_sound", Nbt::Str("minecraft:entity.baby_cat.hiss")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_cat.hurt")),
                        ("purr_sound", Nbt::Str("minecraft:entity.baby_cat.purr")),
                        ("purreow_sound", Nbt::Str("minecraft:entity.baby_cat.purreow")),
                        ("stray_ambient_sound", Nbt::Str("minecraft:entity.baby_cat.stray_ambient")),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn chicken_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:cold",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/chicken/chicken_cold")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/chicken/chicken_cold_baby")),
                ("model", Nbt::Str("cold")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_cold_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:temperate",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/chicken/chicken_temperate")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/chicken/chicken_temperate_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:warm",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/chicken/chicken_warm")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/chicken/chicken_warm_baby")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_warm_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn chicken_sound_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:classic",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.chicken.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.chicken.death")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.chicken.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.chicken.step")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_chicken.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_chicken.death")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_chicken.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_chicken.step")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:picky",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.chicken_picky.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.chicken_picky.death")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.chicken_picky.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.chicken.step")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_chicken.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_chicken.death")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_chicken.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_chicken.step")),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn cow_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:cold",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cow/cow_cold")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cow/cow_cold_baby")),
                ("model", Nbt::Str("cold")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_cold_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:temperate",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cow/cow_temperate")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cow/cow_temperate_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:warm",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/cow/cow_warm")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/cow/cow_warm_baby")),
                ("model", Nbt::Str("warm")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_warm_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn cow_sound_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:classic",
            Nbt::Compound(vec![
                ("ambient_sound", Nbt::Str("minecraft:entity.cow.ambient")),
                ("death_sound", Nbt::Str("minecraft:entity.cow.death")),
                ("hurt_sound", Nbt::Str("minecraft:entity.cow.hurt")),
                ("step_sound", Nbt::Str("minecraft:entity.cow.step")),
            ]),
        ),
        (
            "minecraft:moody",
            Nbt::Compound(vec![
                ("ambient_sound", Nbt::Str("minecraft:entity.cow_moody.ambient")),
                ("death_sound", Nbt::Str("minecraft:entity.cow_moody.death")),
                ("hurt_sound", Nbt::Str("minecraft:entity.cow_moody.hurt")),
                ("step_sound", Nbt::Str("minecraft:entity.cow_moody.step")),
            ]),
        ),
    ]
}

pub fn damage_type() -> Entries<'static> {
    vec![
        (
            "minecraft:arrow",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("arrow")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:bad_respawn_point",
            Nbt::Compound(vec![
                ("death_message_type", Nbt::Str("intentional_game_design")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("badRespawnPoint")),
                ("scaling", Nbt::Str("always")),
            ]),
        ),
        (
            "minecraft:cactus",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("cactus")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:campfire",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("inFire")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:cramming",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("cramming")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:dragon_breath",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("dragonBreath")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:drown",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("drowning")),
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("drown")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:dry_out",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("dryout")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:ender_pearl",
            Nbt::Compound(vec![
                ("death_message_type", Nbt::Str("fall_variants")),
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("fall")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:explosion",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("explosion")),
                ("scaling", Nbt::Str("always")),
            ]),
        ),
        (
            "minecraft:fall",
            Nbt::Compound(vec![
                ("death_message_type", Nbt::Str("fall_variants")),
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("fall")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:falling_anvil",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("anvil")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:falling_block",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("fallingBlock")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:falling_stalactite",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("fallingStalactite")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:fireball",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("fireball")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:fireworks",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("fireworks")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:fly_into_wall",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("flyIntoWall")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:freeze",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("freezing")),
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("freeze")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:generic",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("generic")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:generic_kill",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("genericKill")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:hot_floor",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("hotFloor")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:in_fire",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("inFire")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:in_wall",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("inWall")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:indirect_magic",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("indirectMagic")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:lava",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("lava")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:lightning_bolt",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("lightningBolt")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:mace_smash",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mace_smash")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:magic",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("magic")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:mob_attack",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mob")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:mob_attack_no_aggro",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mob")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:mob_projectile",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mob")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:on_fire",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("onFire")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:out_of_world",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("outOfWorld")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:outside_border",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("outsideBorder")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:player_attack",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("player")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:player_explosion",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("explosion.player")),
                ("scaling", Nbt::Str("always")),
            ]),
        ),
        (
            "minecraft:sonic_boom",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("sonic_boom")),
                ("scaling", Nbt::Str("always")),
            ]),
        ),
        (
            "minecraft:spear",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("spear")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:spit",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mob")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:stalagmite",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("stalagmite")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:starve",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("starve")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:sting",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("sting")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:sulfur_cube_hot",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("sulfurCubeHot")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:sweet_berry_bush",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("poking")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("sweetBerryBush")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:thorns",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("thorns")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("thorns")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:thrown",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("thrown")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:trident",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("trident")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:unattributed_fireball",
            Nbt::Compound(vec![
                ("effects", Nbt::Str("burning")),
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("onFire")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:wind_charge",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("mob")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:wither",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.0_f32)),
                ("message_id", Nbt::Str("wither")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
        (
            "minecraft:wither_skull",
            Nbt::Compound(vec![
                ("exhaustion", Nbt::Float(0.1_f32)),
                ("message_id", Nbt::Str("witherSkull")),
                ("scaling", Nbt::Str("when_caused_by_living_non_player")),
            ]),
        ),
    ]
}

pub fn dimension_type() -> Entries<'static> {
    vec![
        (
            "minecraft:overworld",
            Nbt::Compound(vec![
                ("ambient_light", Nbt::Float(0.0_f32)),
                (
                    "attributes",
                    Nbt::Compound(vec![
                        (
                            "minecraft:audio/ambient_sounds",
                            Nbt::Compound(vec![(
                                "mood",
                                Nbt::Compound(vec![
                                    ("block_search_extent", Nbt::Int(8)),
                                    ("offset", Nbt::Double(2.0)),
                                    ("sound", Nbt::Str("minecraft:ambient.cave")),
                                    ("tick_delay", Nbt::Int(6000)),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:audio/background_music",
                            Nbt::Compound(vec![
                                (
                                    "creative",
                                    Nbt::Compound(vec![
                                        ("max_delay", Nbt::Int(24000)),
                                        ("min_delay", Nbt::Int(12000)),
                                        ("sound", Nbt::Str("minecraft:music.creative")),
                                    ]),
                                ),
                                (
                                    "default",
                                    Nbt::Compound(vec![
                                        ("max_delay", Nbt::Int(24000)),
                                        ("min_delay", Nbt::Int(12000)),
                                        ("sound", Nbt::Str("minecraft:music.game")),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("always")),
                                ("can_sleep", Nbt::Str("when_dark")),
                                ("error_message", Nbt::Compound(vec![("translate", Nbt::Str("block.minecraft.bed.no_sleep"))])),
                            ]),
                        ),
                        ("minecraft:gameplay/nether_portal_spawns_piglin", Nbt::Bool(true)),
                        ("minecraft:gameplay/respawn_anchor_works", Nbt::Bool(false)),
                        (
                            "minecraft:gameplay/straw_bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("when_dark")),
                                ("destroy_on_leave", Nbt::Bool(true)),
                                ("error_message", Nbt::Compound(vec![("translate", Nbt::Str("block.minecraft.bed.no_sleep"))])),
                            ]),
                        ),
                        ("minecraft:visual/ambient_light_color", Nbt::Int(657930)),
                        ("minecraft:visual/cloud_color", Nbt::Int(-855638017)),
                        ("minecraft:visual/cloud_height", Nbt::Double(192.33)),
                        ("minecraft:visual/fog_color", Nbt::Int(12638463)),
                        ("minecraft:visual/sky_color", Nbt::Int(7907327)),
                    ]),
                ),
                ("coordinate_scale", Nbt::Double(1.0)),
                ("default_clock", Nbt::Str("minecraft:overworld")),
                ("has_ceiling", Nbt::Bool(false)),
                ("has_ender_dragon_fight", Nbt::Bool(false)),
                ("has_skylight", Nbt::Bool(true)),
                ("height", Nbt::Int(384)),
                ("infiniburn", Nbt::Str("#minecraft:infiniburn_overworld")),
                ("logical_height", Nbt::Int(384)),
                ("min_y", Nbt::Int(-64)),
                ("monster_spawn_block_light_limit", Nbt::Int(0)),
                (
                    "monster_spawn_light_level",
                    Nbt::Compound(vec![
                        ("type", Nbt::Str("minecraft:uniform")),
                        ("max_inclusive", Nbt::Int(7)),
                        ("min_inclusive", Nbt::Int(0)),
                    ]),
                ),
                ("timelines", Nbt::Str("#minecraft:in_overworld")),
            ]),
        ),
        (
            "minecraft:overworld_caves",
            Nbt::Compound(vec![
                ("ambient_light", Nbt::Float(0.0_f32)),
                (
                    "attributes",
                    Nbt::Compound(vec![
                        (
                            "minecraft:audio/ambient_sounds",
                            Nbt::Compound(vec![(
                                "mood",
                                Nbt::Compound(vec![
                                    ("block_search_extent", Nbt::Int(8)),
                                    ("offset", Nbt::Double(2.0)),
                                    ("sound", Nbt::Str("minecraft:ambient.cave")),
                                    ("tick_delay", Nbt::Int(6000)),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:audio/background_music",
                            Nbt::Compound(vec![
                                (
                                    "creative",
                                    Nbt::Compound(vec![
                                        ("max_delay", Nbt::Int(24000)),
                                        ("min_delay", Nbt::Int(12000)),
                                        ("sound", Nbt::Str("minecraft:music.creative")),
                                    ]),
                                ),
                                (
                                    "default",
                                    Nbt::Compound(vec![
                                        ("max_delay", Nbt::Int(24000)),
                                        ("min_delay", Nbt::Int(12000)),
                                        ("sound", Nbt::Str("minecraft:music.game")),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("always")),
                                ("can_sleep", Nbt::Str("when_dark")),
                                ("error_message", Nbt::Compound(vec![("translate", Nbt::Str("block.minecraft.bed.no_sleep"))])),
                            ]),
                        ),
                        ("minecraft:gameplay/nether_portal_spawns_piglin", Nbt::Bool(true)),
                        ("minecraft:gameplay/respawn_anchor_works", Nbt::Bool(false)),
                        (
                            "minecraft:gameplay/straw_bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("when_dark")),
                                ("destroy_on_leave", Nbt::Bool(true)),
                                ("error_message", Nbt::Compound(vec![("translate", Nbt::Str("block.minecraft.bed.no_sleep"))])),
                            ]),
                        ),
                        ("minecraft:visual/ambient_light_color", Nbt::Int(657930)),
                        ("minecraft:visual/cloud_color", Nbt::Int(-855638017)),
                        ("minecraft:visual/cloud_height", Nbt::Double(192.33)),
                        ("minecraft:visual/fog_color", Nbt::Int(12638463)),
                        ("minecraft:visual/sky_color", Nbt::Int(7907327)),
                    ]),
                ),
                ("coordinate_scale", Nbt::Double(1.0)),
                ("default_clock", Nbt::Str("minecraft:overworld")),
                ("has_ceiling", Nbt::Bool(true)),
                ("has_ender_dragon_fight", Nbt::Bool(false)),
                ("has_skylight", Nbt::Bool(true)),
                ("height", Nbt::Int(384)),
                ("infiniburn", Nbt::Str("#minecraft:infiniburn_overworld")),
                ("logical_height", Nbt::Int(384)),
                ("min_y", Nbt::Int(-64)),
                ("monster_spawn_block_light_limit", Nbt::Int(0)),
                (
                    "monster_spawn_light_level",
                    Nbt::Compound(vec![
                        ("type", Nbt::Str("minecraft:uniform")),
                        ("max_inclusive", Nbt::Int(7)),
                        ("min_inclusive", Nbt::Int(0)),
                    ]),
                ),
                ("timelines", Nbt::Str("#minecraft:in_overworld")),
            ]),
        ),
        (
            "minecraft:the_nether",
            Nbt::Compound(vec![
                ("ambient_light", Nbt::Float(0.1_f32)),
                (
                    "attributes",
                    Nbt::Compound(vec![
                        (
                            "minecraft:gameplay/bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("never")),
                                ("destroy_on_use", Nbt::Bool(true)),
                            ]),
                        ),
                        ("minecraft:gameplay/can_start_raid", Nbt::Bool(false)),
                        ("minecraft:gameplay/fast_lava", Nbt::Bool(true)),
                        ("minecraft:gameplay/piglins_zombify", Nbt::Bool(false)),
                        ("minecraft:gameplay/respawn_anchor_works", Nbt::Bool(true)),
                        ("minecraft:gameplay/sky_light_level", Nbt::Double(4.0)),
                        ("minecraft:gameplay/snow_golem_melts", Nbt::Bool(true)),
                        (
                            "minecraft:gameplay/straw_bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("never")),
                                ("destroy_on_use", Nbt::Bool(true)),
                            ]),
                        ),
                        ("minecraft:gameplay/water_evaporates", Nbt::Bool(true)),
                        ("minecraft:visual/ambient_light_color", Nbt::Int(3156001)),
                        (
                            "minecraft:visual/default_dripstone_particle",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:dripping_dripstone_lava"))]),
                        ),
                        ("minecraft:visual/fog_end_distance", Nbt::Double(96.0)),
                        ("minecraft:visual/fog_start_distance", Nbt::Double(1e+01)),
                        ("minecraft:visual/sky_light_color", Nbt::Int(8026879)),
                        ("minecraft:visual/sky_light_factor", Nbt::Float(0.0_f32)),
                    ]),
                ),
                ("cardinal_light", Nbt::Str("nether")),
                ("coordinate_scale", Nbt::Double(8.0)),
                ("has_ceiling", Nbt::Bool(true)),
                ("has_ender_dragon_fight", Nbt::Bool(false)),
                ("has_fixed_time", Nbt::Bool(true)),
                ("has_skylight", Nbt::Bool(false)),
                ("height", Nbt::Int(256)),
                ("infiniburn", Nbt::Str("#minecraft:infiniburn_nether")),
                ("logical_height", Nbt::Int(128)),
                ("min_y", Nbt::Int(0)),
                ("monster_spawn_block_light_limit", Nbt::Int(15)),
                ("monster_spawn_light_level", Nbt::Int(7)),
                ("skybox", Nbt::Str("none")),
                ("timelines", Nbt::Str("#minecraft:in_nether")),
            ]),
        ),
        (
            "minecraft:the_end",
            Nbt::Compound(vec![
                ("ambient_light", Nbt::Float(0.25_f32)),
                (
                    "attributes",
                    Nbt::Compound(vec![
                        (
                            "minecraft:audio/ambient_sounds",
                            Nbt::Compound(vec![(
                                "mood",
                                Nbt::Compound(vec![
                                    ("block_search_extent", Nbt::Int(8)),
                                    ("offset", Nbt::Double(2.0)),
                                    ("sound", Nbt::Str("minecraft:ambient.cave")),
                                    ("tick_delay", Nbt::Int(6000)),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:audio/background_music",
                            Nbt::Compound(vec![(
                                "default",
                                Nbt::Compound(vec![
                                    ("max_delay", Nbt::Int(24000)),
                                    ("min_delay", Nbt::Int(6000)),
                                    ("replace_current_music", Nbt::Bool(true)),
                                    ("sound", Nbt::Str("minecraft:music.end")),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:gameplay/bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("never")),
                                ("destroy_on_use", Nbt::Bool(true)),
                            ]),
                        ),
                        ("minecraft:gameplay/respawn_anchor_works", Nbt::Bool(false)),
                        (
                            "minecraft:gameplay/straw_bed_rule",
                            Nbt::Compound(vec![
                                ("can_set_spawn", Nbt::Str("never")),
                                ("can_sleep", Nbt::Str("never")),
                                ("destroy_on_use", Nbt::Bool(true)),
                            ]),
                        ),
                        ("minecraft:visual/ambient_light_color", Nbt::Int(4147007)),
                        ("minecraft:visual/fog_color", Nbt::Int(1577752)),
                        ("minecraft:visual/sky_color", Nbt::Int(0)),
                        ("minecraft:visual/sky_light_color", Nbt::Int(11296973)),
                        ("minecraft:visual/sky_light_factor", Nbt::Float(0.0_f32)),
                    ]),
                ),
                ("coordinate_scale", Nbt::Double(1.0)),
                ("default_clock", Nbt::Str("minecraft:the_end")),
                ("has_ceiling", Nbt::Bool(false)),
                ("has_ender_dragon_fight", Nbt::Bool(true)),
                ("has_fixed_time", Nbt::Bool(true)),
                ("has_skylight", Nbt::Bool(true)),
                ("height", Nbt::Int(256)),
                ("infiniburn", Nbt::Str("#minecraft:infiniburn_end")),
                ("logical_height", Nbt::Int(256)),
                ("min_y", Nbt::Int(0)),
                ("monster_spawn_block_light_limit", Nbt::Int(0)),
                ("monster_spawn_light_level", Nbt::Int(15)),
                ("skybox", Nbt::Str("end")),
                ("timelines", Nbt::Str("#minecraft:in_end")),
            ]),
        ),
    ]
}

pub fn frog_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:cold",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/frog/frog_cold")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_cold_variant_frogs")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:temperate",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/frog/frog_temperate")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:warm",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/frog/frog_warm")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_warm_variant_frogs")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn instrument() -> Entries<'static> {
    vec![
        (
            "minecraft:admire_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.admire_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.4")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:call_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.call_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.5")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:dream_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.dream_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.7")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:feel_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.feel_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.3")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:ponder_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.ponder_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.0")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:seek_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.seek_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.2")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:sing_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.sing_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.1")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
        (
            "minecraft:yearn_goat_horn",
            Nbt::Compound(vec![
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("instrument.minecraft.yearn_goat_horn"))])),
                ("range", Nbt::Float(256.0_f32)),
                ("sound_event", Nbt::Str("minecraft:item.goat_horn.sound.6")),
                ("use_duration", Nbt::Float(7.0_f32)),
            ]),
        ),
    ]
}

pub fn jukebox_song() -> Entries<'static> {
    vec![
        (
            "minecraft:11",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(11)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.11"))])),
                ("length_in_seconds", Nbt::Float(71.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.11")),
            ]),
        ),
        (
            "minecraft:13",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(1)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.13"))])),
                ("length_in_seconds", Nbt::Float(178.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.13")),
            ]),
        ),
        (
            "minecraft:5",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(15)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.5"))])),
                ("length_in_seconds", Nbt::Float(178.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.5")),
            ]),
        ),
        (
            "minecraft:blocks",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(3)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.blocks"))])),
                ("length_in_seconds", Nbt::Float(345.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.blocks")),
            ]),
        ),
        (
            "minecraft:bounce",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(8)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.bounce"))])),
                ("length_in_seconds", Nbt::Float(234.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.bounce")),
            ]),
        ),
        (
            "minecraft:cat",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(2)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.cat"))])),
                ("length_in_seconds", Nbt::Float(185.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.cat")),
            ]),
        ),
        (
            "minecraft:chirp",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(4)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.chirp"))])),
                ("length_in_seconds", Nbt::Float(185.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.chirp")),
            ]),
        ),
        (
            "minecraft:creator",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(12)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.creator"))])),
                ("length_in_seconds", Nbt::Float(176.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.creator")),
            ]),
        ),
        (
            "minecraft:creator_music_box",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(11)),
                (
                    "description",
                    Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.creator_music_box"))]),
                ),
                ("length_in_seconds", Nbt::Float(73.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.creator_music_box")),
            ]),
        ),
        (
            "minecraft:far",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(5)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.far"))])),
                ("length_in_seconds", Nbt::Float(174.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.far")),
            ]),
        ),
        (
            "minecraft:lava_chicken",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(9)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.lava_chicken"))])),
                ("length_in_seconds", Nbt::Float(134.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.lava_chicken")),
            ]),
        ),
        (
            "minecraft:mall",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(6)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.mall"))])),
                ("length_in_seconds", Nbt::Float(197.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.mall")),
            ]),
        ),
        (
            "minecraft:mellohi",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(7)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.mellohi"))])),
                ("length_in_seconds", Nbt::Float(96.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.mellohi")),
            ]),
        ),
        (
            "minecraft:otherside",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(14)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.otherside"))])),
                ("length_in_seconds", Nbt::Float(195.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.otherside")),
            ]),
        ),
        (
            "minecraft:pigstep",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(13)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.pigstep"))])),
                ("length_in_seconds", Nbt::Float(149.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.pigstep")),
            ]),
        ),
        (
            "minecraft:precipice",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(13)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.precipice"))])),
                ("length_in_seconds", Nbt::Float(299.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.precipice")),
            ]),
        ),
        (
            "minecraft:relic",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(14)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.relic"))])),
                ("length_in_seconds", Nbt::Float(218.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.relic")),
            ]),
        ),
        (
            "minecraft:stal",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(8)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.stal"))])),
                ("length_in_seconds", Nbt::Float(1.5e+02_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.stal")),
            ]),
        ),
        (
            "minecraft:strad",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(9)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.strad"))])),
                ("length_in_seconds", Nbt::Float(188.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.strad")),
            ]),
        ),
        (
            "minecraft:tears",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(10)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.tears"))])),
                ("length_in_seconds", Nbt::Float(175.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.tears")),
            ]),
        ),
        (
            "minecraft:wait",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(12)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.wait"))])),
                ("length_in_seconds", Nbt::Float(238.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.wait")),
            ]),
        ),
        (
            "minecraft:ward",
            Nbt::Compound(vec![
                ("comparator_output", Nbt::Int(10)),
                ("description", Nbt::Compound(vec![("translate", Nbt::Str("jukebox_song.minecraft.ward"))])),
                ("length_in_seconds", Nbt::Float(251.0_f32)),
                ("sound_event", Nbt::Str("minecraft:music_disc.ward")),
            ]),
        ),
    ]
}

pub fn painting_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:alban",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:alban")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.alban.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.alban.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:aztec",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:aztec")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.aztec.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.aztec.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:aztec2",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:aztec2")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.aztec2.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.aztec2.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:backyard",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:backyard")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.backyard.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.backyard.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:baroque",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:baroque")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.baroque.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.baroque.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:bomb",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:bomb")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.bomb.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.bomb.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:bouquet",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:bouquet")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.bouquet.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.bouquet.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:burning_skull",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:burning_skull")),
                (
                    "author",
                    Nbt::Compound(vec![
                        ("color", Nbt::Str("gray")),
                        ("translate", Nbt::Str("painting.minecraft.burning_skull.author")),
                    ]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![
                        ("color", Nbt::Str("yellow")),
                        ("translate", Nbt::Str("painting.minecraft.burning_skull.title")),
                    ]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:bust",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:bust")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.bust.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.bust.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:cavebird",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:cavebird")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.cavebird.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.cavebird.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:changing",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:changing")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.changing.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.changing.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:cotan",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:cotan")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.cotan.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.cotan.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:courbet",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:courbet")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.courbet.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.courbet.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:creebet",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:creebet")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.creebet.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.creebet.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:dennis",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:dennis")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.dennis.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.dennis.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:donkey_kong",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:donkey_kong")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.donkey_kong.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.donkey_kong.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:earth",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:earth")),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.earth.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:endboss",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:endboss")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.endboss.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.endboss.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:fern",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:fern")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.fern.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.fern.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:fighters",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:fighters")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.fighters.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.fighters.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:finding",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:finding")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.finding.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.finding.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:fire",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:fire")),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.fire.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:graham",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:graham")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.graham.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.graham.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:humble",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:humble")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.humble.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.humble.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:kebab",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:kebab")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.kebab.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.kebab.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:lowmist",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:lowmist")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.lowmist.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.lowmist.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:match",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:match")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.match.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.match.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:meditative",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:meditative")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.meditative.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.meditative.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:orb",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:orb")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.orb.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.orb.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:owlemons",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:owlemons")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.owlemons.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.owlemons.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:passage",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:passage")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.passage.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.passage.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:pigscene",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:pigscene")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.pigscene.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.pigscene.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:plant",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:plant")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.plant.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.plant.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:pointer",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:pointer")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.pointer.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.pointer.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:pond",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:pond")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.pond.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.pond.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:pool",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:pool")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.pool.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.pool.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:prairie_ride",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:prairie_ride")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.prairie_ride.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![
                        ("color", Nbt::Str("yellow")),
                        ("translate", Nbt::Str("painting.minecraft.prairie_ride.title")),
                    ]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:sea",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:sea")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.sea.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.sea.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:skeleton",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:skeleton")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.skeleton.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.skeleton.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:skull_and_roses",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:skull_and_roses")),
                (
                    "author",
                    Nbt::Compound(vec![
                        ("color", Nbt::Str("gray")),
                        ("translate", Nbt::Str("painting.minecraft.skull_and_roses.author")),
                    ]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![
                        ("color", Nbt::Str("yellow")),
                        ("translate", Nbt::Str("painting.minecraft.skull_and_roses.title")),
                    ]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:stage",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:stage")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.stage.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.stage.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:sunflowers",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:sunflowers")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.sunflowers.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.sunflowers.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:sunset",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:sunset")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.sunset.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.sunset.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:tides",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:tides")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.tides.author"))]),
                ),
                ("height", Nbt::Int(3)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.tides.title"))]),
                ),
                ("width", Nbt::Int(3)),
            ]),
        ),
        (
            "minecraft:unpacked",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:unpacked")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.unpacked.author"))]),
                ),
                ("height", Nbt::Int(4)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.unpacked.title"))]),
                ),
                ("width", Nbt::Int(4)),
            ]),
        ),
        (
            "minecraft:void",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:void")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.void.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.void.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:wanderer",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:wanderer")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.wanderer.author"))]),
                ),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.wanderer.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:wasteland",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:wasteland")),
                (
                    "author",
                    Nbt::Compound(vec![("color", Nbt::Str("gray")), ("translate", Nbt::Str("painting.minecraft.wasteland.author"))]),
                ),
                ("height", Nbt::Int(1)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.wasteland.title"))]),
                ),
                ("width", Nbt::Int(1)),
            ]),
        ),
        (
            "minecraft:water",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:water")),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.water.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:wind",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:wind")),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.wind.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
        (
            "minecraft:wither",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:wither")),
                ("height", Nbt::Int(2)),
                (
                    "title",
                    Nbt::Compound(vec![("color", Nbt::Str("yellow")), ("translate", Nbt::Str("painting.minecraft.wither.title"))]),
                ),
                ("width", Nbt::Int(2)),
            ]),
        ),
    ]
}

pub fn decorated_pot_pattern() -> Entries<'static> {
    vec![
        ("minecraft:angler", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:angler_pottery_pattern"))])),
        ("minecraft:archer", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:archer_pottery_pattern"))])),
        (
            "minecraft:arms_up",
            Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:arms_up_pottery_pattern"))]),
        ),
        ("minecraft:blade", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:blade_pottery_pattern"))])),
        ("minecraft:brewer", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:brewer_pottery_pattern"))])),
        ("minecraft:burn", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:burn_pottery_pattern"))])),
        ("minecraft:danger", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:danger_pottery_pattern"))])),
        (
            "minecraft:explorer",
            Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:explorer_pottery_pattern"))]),
        ),
        ("minecraft:flow", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:flow_pottery_pattern"))])),
        ("minecraft:friend", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:friend_pottery_pattern"))])),
        ("minecraft:guster", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:guster_pottery_pattern"))])),
        ("minecraft:heart", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:heart_pottery_pattern"))])),
        (
            "minecraft:heartbreak",
            Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:heartbreak_pottery_pattern"))]),
        ),
        ("minecraft:howl", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:howl_pottery_pattern"))])),
        ("minecraft:miner", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:miner_pottery_pattern"))])),
        (
            "minecraft:mourner",
            Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:mourner_pottery_pattern"))]),
        ),
        ("minecraft:plenty", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:plenty_pottery_pattern"))])),
        ("minecraft:prize", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:prize_pottery_pattern"))])),
        ("minecraft:scrape", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:scrape_pottery_pattern"))])),
        ("minecraft:sheaf", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:sheaf_pottery_pattern"))])),
        (
            "minecraft:shelter",
            Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:shelter_pottery_pattern"))]),
        ),
        ("minecraft:skull", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:skull_pottery_pattern"))])),
        ("minecraft:snort", Nbt::Compound(vec![("asset_id", Nbt::Str("minecraft:snort_pottery_pattern"))])),
    ]
}

pub fn pig_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:cold",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/pig/pig_cold")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/pig/pig_cold_baby")),
                ("model", Nbt::Str("cold")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_cold_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:temperate",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/pig/pig_temperate")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/pig/pig_temperate_baby")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:warm",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/pig/pig_warm")),
                ("baby_asset_id", Nbt::Str("minecraft:entity/pig/pig_warm_baby")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_warm_variant_farm_animals")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn pig_sound_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:big",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.pig_big.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.pig_big.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.pig_big.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.pig_big.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.pig.step")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_pig.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_pig.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.baby_pig.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_pig.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_pig.step")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:classic",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.pig.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.pig.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.pig.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.pig.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.pig.step")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_pig.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_pig.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.baby_pig.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_pig.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_pig.step")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:mini",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.pig_mini.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.pig_mini.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.pig_mini.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.pig_mini.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.pig.step")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_pig.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_pig.death")),
                        ("eat_sound", Nbt::Str("minecraft:entity.baby_pig.eat")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_pig.hurt")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_pig.step")),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn sulfur_cube_archetype() -> Entries<'static> {
    vec![
        (
            "minecraft:bouncy",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-2.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:bouncy_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-2.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:bouncy_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.9_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:bouncy_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:bouncy_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:bouncy_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("buoyant", Nbt::Bool(true)),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/bouncy")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.105_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.bouncy.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.bouncy.push")),
                        ("push_sound_cooldown", Nbt::Float(0.7_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.3_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:explosive",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:explosive_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:explosive_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:explosive_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:explosive_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:explosive_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("buoyant", Nbt::Bool(true)),
                (
                    "explosion",
                    Nbt::Compound(vec![("causes_fire", Nbt::Bool(false)), ("fuse", Nbt::Int(120)), ("power", Nbt::Int(3))]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/explosive")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.explosive.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.explosive.push")),
                        ("push_sound_cooldown", Nbt::Float(0.7_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.1_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:fast_flat",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:fast_flat_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:fast_flat_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:fast_flat_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.8_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:fast_flat_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:fast_flat_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/fast_flat")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.9125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.fast_flat.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.fast_flat.push")),
                        ("push_sound_cooldown", Nbt::Float(0.9_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.03_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:fast_sliding",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:fast_sliding_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:fast_sliding_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.1_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:fast_sliding_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.95_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:fast_sliding_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:fast_sliding_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/fast_sliding")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.6625_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.fast_sliding.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.fast_sliding.push")),
                        ("push_sound_cooldown", Nbt::Float(1.0_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.05_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:high_resistance",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:high_resistance_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:high_resistance_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.2_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:high_resistance_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.0_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:high_resistance_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:high_resistance_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/high_resistance")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.high_resistance.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.high_resistance.push")),
                        ("push_sound_cooldown", Nbt::Float(0.7_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.03_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:hot",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:hot_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:hot_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:hot_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:hot_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.9_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:hot_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("buoyant", Nbt::Bool(true)),
                (
                    "contact_damage",
                    Nbt::Compound(vec![
                        ("amount", Nbt::Float(1.0_f32)),
                        ("attribute_to_source", Nbt::Bool(false)),
                        ("damage_type", Nbt::Str("minecraft:sulfur_cube_hot")),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/hot")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.hot.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.hot.push")),
                        ("push_sound_cooldown", Nbt::Float(0.7_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.2_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:light",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:light_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:light_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:light_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:light_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.79999995_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:light_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("buoyant", Nbt::Bool(true)),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/light")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.18_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.light.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.light.push")),
                        ("push_sound_cooldown", Nbt::Float(0.7_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.2_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:regular",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:regular_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:regular_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:regular_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:regular_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.9_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:regular_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("buoyant", Nbt::Bool(true)),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/regular")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.regular.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.regular.push")),
                        ("push_sound_cooldown", Nbt::Float(0.5_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.2_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:slow_bouncy",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.4_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_bouncy_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.4_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_bouncy_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.6_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:slow_bouncy_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.7_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:slow_bouncy_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.95_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:slow_bouncy_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/slow_bouncy")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.24_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_bouncy.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_bouncy.push")),
                        ("push_sound_cooldown", Nbt::Float(0.5_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.05_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:slow_flat",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_flat_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.5_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_flat_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.4_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:slow_flat_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.6_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:slow_flat_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.9_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:slow_flat_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/slow_flat")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.105_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_flat.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_flat.push")),
                        ("push_sound_cooldown", Nbt::Float(0.9_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.03_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:slow_sliding",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.8_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_sliding_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.8_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:slow_sliding_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.1_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:slow_sliding_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.95_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:slow_sliding_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:slow_sliding_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/slow_sliding")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_sliding.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.slow_sliding.push")),
                        ("push_sound_cooldown", Nbt::Float(1.0_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.02_f32)),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:sticky",
            Nbt::Compound(vec![
                (
                    "attribute_modifiers",
                    Nbt::List(vec![
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-2.0_f32)),
                            ("attribute", Nbt::Str("minecraft:knockback_resistance")),
                            ("id", Nbt::Str("minecraft:sticky_add_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-2.0_f32)),
                            ("attribute", Nbt::Str("minecraft:explosion_knockback_resistance")),
                            ("id", Nbt::Str("minecraft:sticky_add_explosion_knockback_resistance")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(0.0_f32)),
                            ("attribute", Nbt::Str("minecraft:bounciness")),
                            ("id", Nbt::Str("minecraft:sticky_add_bounciness")),
                            ("operation", Nbt::Str("add_value")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(1.0_f32)),
                            ("attribute", Nbt::Str("minecraft:friction_modifier")),
                            ("id", Nbt::Str("minecraft:sticky_mul_friction_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                        Nbt::Compound(vec![
                            ("amount", Nbt::Float(-0.99_f32)),
                            ("attribute", Nbt::Str("minecraft:air_drag_modifier")),
                            ("id", Nbt::Str("minecraft:sticky_mul_air_drag_modifier")),
                            ("operation", Nbt::Str("add_multiplied_total")),
                        ]),
                    ]),
                ),
                ("items", Nbt::Str("#minecraft:sulfur_cube_archetype/sticky")),
                (
                    "knockback_modifiers",
                    Nbt::Compound(vec![("horizontal_power", Nbt::Float(0.4125_f32)), ("vertical_power", Nbt::Float(0.09_f32))]),
                ),
                (
                    "sound_settings",
                    Nbt::Compound(vec![
                        ("hit_sound", Nbt::Str("minecraft:entity.sulfur_cube.sticky.hit")),
                        ("push_sound", Nbt::Str("minecraft:entity.sulfur_cube.sticky.push")),
                        ("push_sound_cooldown", Nbt::Float(0.5_f32)),
                        ("push_sound_impulse_threshold", Nbt::Float(0.05_f32)),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn timeline() -> Entries<'static> {
    vec![
        (
            "minecraft:day",
            Nbt::Compound(vec![
                ("clock", Nbt::Str("minecraft:overworld")),
                ("period_ticks", Nbt::Int(24000)),
                (
                    "time_markers",
                    Nbt::Compound(vec![
                        (
                            "minecraft:day",
                            Nbt::Compound(vec![("show_in_commands", Nbt::Bool(true)), ("ticks", Nbt::Int(1000))]),
                        ),
                        (
                            "minecraft:midnight",
                            Nbt::Compound(vec![("show_in_commands", Nbt::Bool(true)), ("ticks", Nbt::Int(18000))]),
                        ),
                        (
                            "minecraft:night",
                            Nbt::Compound(vec![("show_in_commands", Nbt::Bool(true)), ("ticks", Nbt::Int(13000))]),
                        ),
                        (
                            "minecraft:noon",
                            Nbt::Compound(vec![("show_in_commands", Nbt::Bool(true)), ("ticks", Nbt::Int(6000))]),
                        ),
                        ("minecraft:roll_village_siege", Nbt::Int(18000)),
                        ("minecraft:wake_up_from_sleep", Nbt::Int(0)),
                    ]),
                ),
                (
                    "tracks",
                    Nbt::Compound(vec![
                        (
                            "minecraft:audio/firefly_bush_sounds",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12600)), ("value", Nbt::Bool(true))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23401)), ("value", Nbt::Bool(false))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("or")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/bees_stay_in_hive",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12542)), ("value", Nbt::Bool(true))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23460)), ("value", Nbt::Bool(false))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("or")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/cat_waking_up_gift_chance",
                            Nbt::Compound(vec![
                                ("ease", Nbt::Str("constant")),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(362)), ("value", Nbt::Float(0.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23667)), ("value", Nbt::Float(0.7_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("maximum")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/creaking_active",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12600)), ("value", Nbt::Bool(true))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23401)), ("value", Nbt::Bool(false))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("or")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/eyeblossom_open",
                            Nbt::Compound(vec![(
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12600)), ("value", Nbt::Bool(true))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23401)), ("value", Nbt::Bool(false))]),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:gameplay/monsters_burn",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12542)), ("value", Nbt::Bool(false))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23460)), ("value", Nbt::Bool(true))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("or")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/sky_light_level",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(133)), ("value", Nbt::Float(1.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11867)), ("value", Nbt::Float(1.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13670)), ("value", Nbt::Float(0.26666668_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22330)), ("value", Nbt::Float(0.26666668_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:gameplay/turtle_egg_hatch_chance",
                            Nbt::Compound(vec![
                                ("ease", Nbt::Str("constant")),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(21062)), ("value", Nbt::Float(1.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(21905)), ("value", Nbt::Float(0.002_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("maximum")),
                            ]),
                        ),
                        (
                            "minecraft:visual/cloud_color",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(133)), ("value", Nbt::Str("#ffffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11867)), ("value", Nbt::Str("#ffffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13670)), ("value", Nbt::Str("#ff191926"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22330)), ("value", Nbt::Str("#ff191926"))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:visual/fog_color",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(133)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11867)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13670)), ("value", Nbt::Str("#0f0f16"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22330)), ("value", Nbt::Str("#0f0f16"))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:visual/moon_angle",
                            Nbt::Compound(vec![
                                (
                                    "ease",
                                    Nbt::Compound(vec![(
                                        "cubic_bezier",
                                        Nbt::List(vec![
                                            Nbt::Float(0.362_f32),
                                            Nbt::Float(0.241_f32),
                                            Nbt::Float(0.638_f32),
                                            Nbt::Float(0.759_f32),
                                        ]),
                                    )]),
                                ),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(5.4e+02_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(1.8e+02_f32))]),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            "minecraft:visual/sky_color",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(133)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11867)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13670)), ("value", Nbt::Str("#000000"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22330)), ("value", Nbt::Str("#000000"))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:visual/sky_light_color",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(730)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11270)), ("value", Nbt::Str("#ffffff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13140)), ("value", Nbt::Str("#7a7aff"))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22860)), ("value", Nbt::Str("#7a7aff"))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:visual/sky_light_factor",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(730)), ("value", Nbt::Float(1.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11270)), ("value", Nbt::Float(1.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13140)), ("value", Nbt::Float(0.24_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22860)), ("value", Nbt::Float(0.24_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("multiply")),
                            ]),
                        ),
                        (
                            "minecraft:visual/star_angle",
                            Nbt::Compound(vec![
                                (
                                    "ease",
                                    Nbt::Compound(vec![(
                                        "cubic_bezier",
                                        Nbt::List(vec![
                                            Nbt::Float(0.362_f32),
                                            Nbt::Float(0.241_f32),
                                            Nbt::Float(0.638_f32),
                                            Nbt::Float(0.759_f32),
                                        ]),
                                    )]),
                                ),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(3.6e+02_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(0.0_f32))]),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            "minecraft:visual/star_brightness",
                            Nbt::Compound(vec![
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(92)), ("value", Nbt::Float(0.037_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(627)), ("value", Nbt::Float(0.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11373)), ("value", Nbt::Float(0.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11732)), ("value", Nbt::Float(0.016_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(11959)), ("value", Nbt::Float(0.044_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12399)), ("value", Nbt::Float(0.143_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(12729)), ("value", Nbt::Float(0.258_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(13228)), ("value", Nbt::Float(0.5_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(22772)), ("value", Nbt::Float(0.5_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23032)), ("value", Nbt::Float(0.364_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23356)), ("value", Nbt::Float(0.225_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(23758)), ("value", Nbt::Float(0.101_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("maximum")),
                            ]),
                        ),
                        (
                            "minecraft:visual/sun_angle",
                            Nbt::Compound(vec![
                                (
                                    "ease",
                                    Nbt::Compound(vec![(
                                        "cubic_bezier",
                                        Nbt::List(vec![
                                            Nbt::Float(0.362_f32),
                                            Nbt::Float(0.241_f32),
                                            Nbt::Float(0.638_f32),
                                            Nbt::Float(0.759_f32),
                                        ]),
                                    )]),
                                ),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(3.6e+02_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Float(0.0_f32))]),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            "minecraft:visual/sunrise_sunset_color",
                            Nbt::Compound(vec![(
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(71)), ("value", Nbt::Str("#5fefa333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(310)), ("value", Nbt::Str("#29f5ba33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(565)), ("value", Nbt::Str("#06fbd433"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(730)), ("value", Nbt::Str("#00ffe533"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11270)), ("value", Nbt::Str("#00ffe533"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11397)), ("value", Nbt::Str("#04fcd833"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11522)), ("value", Nbt::Str("#0ff9cb33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11690)), ("value", Nbt::Str("#29f5ba33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11929)), ("value", Nbt::Str("#5fefa333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12243)), ("value", Nbt::Str("#b1e78733"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12358)), ("value", Nbt::Str("#cce47e33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12512)), ("value", Nbt::Str("#e9e07233"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12613)), ("value", Nbt::Str("#f6dd6b33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12732)), ("value", Nbt::Str("#feda6333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12841)), ("value", Nbt::Str("#fed75c33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(13035)), ("value", Nbt::Str("#ecd25133"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(13252)), ("value", Nbt::Str("#c1cc4733"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(13775)), ("value", Nbt::Str("#36be3733"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(13888)), ("value", Nbt::Str("#1fbb3533"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(14039)), ("value", Nbt::Str("#09b73333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(14192)), ("value", Nbt::Str("#00b33333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(21807)), ("value", Nbt::Str("#00b23333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(21961)), ("value", Nbt::Str("#09b73333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(22112)), ("value", Nbt::Str("#1fbb3533"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(22225)), ("value", Nbt::Str("#36be3733"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(22748)), ("value", Nbt::Str("#c1cc4733"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(22965)), ("value", Nbt::Str("#ecd25133"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23159)), ("value", Nbt::Str("#fed75c33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23272)), ("value", Nbt::Str("#feda6333"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23488)), ("value", Nbt::Str("#e9e07233"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23642)), ("value", Nbt::Str("#cce47e33"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(23757)), ("value", Nbt::Str("#b1e78733"))]),
                                ]),
                            )]),
                        ),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:early_game",
            Nbt::Compound(vec![
                ("clock", Nbt::Str("minecraft:overworld")),
                (
                    "tracks",
                    Nbt::Compound(vec![(
                        "minecraft:gameplay/can_pillager_patrol_spawn",
                        Nbt::Compound(vec![
                            (
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(0)), ("value", Nbt::Bool(false))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(120000)), ("value", Nbt::Bool(true))]),
                                ]),
                            ),
                            ("modifier", Nbt::Str("and")),
                        ]),
                    )]),
                ),
            ]),
        ),
        (
            "minecraft:moon",
            Nbt::Compound(vec![
                ("clock", Nbt::Str("minecraft:overworld")),
                ("period_ticks", Nbt::Int(192000)),
                (
                    "tracks",
                    Nbt::Compound(vec![
                        (
                            "minecraft:gameplay/surface_slime_spawn_chance",
                            Nbt::Compound(vec![
                                ("ease", Nbt::Str("constant")),
                                (
                                    "keyframes",
                                    Nbt::List(vec![
                                        Nbt::Compound(vec![("ticks", Nbt::Int(0)), ("value", Nbt::Float(0.5_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(24000)), ("value", Nbt::Float(0.375_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(48000)), ("value", Nbt::Float(0.25_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(72000)), ("value", Nbt::Float(0.125_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(96000)), ("value", Nbt::Float(0.0_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(120000)), ("value", Nbt::Float(0.125_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(144000)), ("value", Nbt::Float(0.25_f32))]),
                                        Nbt::Compound(vec![("ticks", Nbt::Int(168000)), ("value", Nbt::Float(0.375_f32))]),
                                    ]),
                                ),
                                ("modifier", Nbt::Str("maximum")),
                            ]),
                        ),
                        (
                            "minecraft:visual/moon_phase",
                            Nbt::Compound(vec![(
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(0)), ("value", Nbt::Str("full_moon"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(24000)), ("value", Nbt::Str("waning_gibbous"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(48000)), ("value", Nbt::Str("third_quarter"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(72000)), ("value", Nbt::Str("waning_crescent"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(96000)), ("value", Nbt::Str("new_moon"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(120000)), ("value", Nbt::Str("waxing_crescent"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(144000)), ("value", Nbt::Str("first_quarter"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(168000)), ("value", Nbt::Str("waxing_gibbous"))]),
                                ]),
                            )]),
                        ),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:villager_schedule",
            Nbt::Compound(vec![
                ("clock", Nbt::Str("minecraft:overworld")),
                ("period_ticks", Nbt::Int(24000)),
                (
                    "tracks",
                    Nbt::Compound(vec![
                        (
                            "minecraft:gameplay/baby_villager_activity",
                            Nbt::Compound(vec![(
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(10)), ("value", Nbt::Str("minecraft:idle"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(3000)), ("value", Nbt::Str("minecraft:play"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(6000)), ("value", Nbt::Str("minecraft:idle"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(10000)), ("value", Nbt::Str("minecraft:play"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12000)), ("value", Nbt::Str("minecraft:rest"))]),
                                ]),
                            )]),
                        ),
                        (
                            "minecraft:gameplay/villager_activity",
                            Nbt::Compound(vec![(
                                "keyframes",
                                Nbt::List(vec![
                                    Nbt::Compound(vec![("ticks", Nbt::Int(10)), ("value", Nbt::Str("minecraft:idle"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(2000)), ("value", Nbt::Str("minecraft:work"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(9000)), ("value", Nbt::Str("minecraft:meet"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(11000)), ("value", Nbt::Str("minecraft:idle"))]),
                                    Nbt::Compound(vec![("ticks", Nbt::Int(12000)), ("value", Nbt::Str("minecraft:rest"))]),
                                ]),
                            )]),
                        ),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn trim_material() -> Entries<'static> {
    vec![
        (
            "minecraft:amethyst",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#9A5CC6")), ("translate", Nbt::Str("trim_material.minecraft.amethyst"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/amethyst")),
            ]),
        ),
        (
            "minecraft:copper",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#B4684D")), ("translate", Nbt::Str("trim_material.minecraft.copper"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/copper")),
            ]),
        ),
        (
            "minecraft:diamond",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#6EECD2")), ("translate", Nbt::Str("trim_material.minecraft.diamond"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/diamond")),
            ]),
        ),
        (
            "minecraft:emerald",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#11A036")), ("translate", Nbt::Str("trim_material.minecraft.emerald"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/emerald")),
            ]),
        ),
        (
            "minecraft:gold",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#DEB12D")), ("translate", Nbt::Str("trim_material.minecraft.gold"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/gold")),
            ]),
        ),
        (
            "minecraft:iron",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#ECECEC")), ("translate", Nbt::Str("trim_material.minecraft.iron"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/iron")),
            ]),
        ),
        (
            "minecraft:lapis",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#416E97")), ("translate", Nbt::Str("trim_material.minecraft.lapis"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/lapis")),
            ]),
        ),
        (
            "minecraft:netherite",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#625859")), ("translate", Nbt::Str("trim_material.minecraft.netherite"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/netherite")),
            ]),
        ),
        (
            "minecraft:quartz",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#E3D4C4")), ("translate", Nbt::Str("trim_material.minecraft.quartz"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/quartz")),
            ]),
        ),
        (
            "minecraft:redstone",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#971607")), ("translate", Nbt::Str("trim_material.minecraft.redstone"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/redstone")),
            ]),
        ),
        (
            "minecraft:resin",
            Nbt::Compound(vec![
                (
                    "description",
                    Nbt::Compound(vec![("color", Nbt::Str("#FC7812")), ("translate", Nbt::Str("trim_material.minecraft.resin"))]),
                ),
                ("palette_id", Nbt::Str("minecraft:trim/resin")),
            ]),
        ),
    ]
}

pub fn wolf_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:ashen",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_ashen_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_ashen_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_ashen")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_ashen_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_ashen_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_ashen_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("minecraft:snowy_taiga"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:black",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_black_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_black_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_black")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_black_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_black_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_black_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("minecraft:old_growth_pine_taiga")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:chestnut",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_chestnut_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_chestnut_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_chestnut")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_chestnut_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_chestnut_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_chestnut_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("minecraft:old_growth_spruce_taiga")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:pale",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_baby")),
                    ]),
                ),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:rusty",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_rusty_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_rusty_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_rusty")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_rusty_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_rusty_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_rusty_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("#minecraft:is_jungle"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:snowy",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_snowy_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_snowy_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_snowy")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_snowy_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_snowy_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_snowy_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("minecraft:grove"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:spotted",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_spotted_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_spotted_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_spotted")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_spotted_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_spotted_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_spotted_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("#minecraft:is_savanna"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:striped",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_striped_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_striped_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_striped")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_striped_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_striped_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_striped_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("#minecraft:is_badlands"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
        (
            "minecraft:woods",
            Nbt::Compound(vec![
                (
                    "assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_woods_angry")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_woods_tame")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_woods")),
                    ]),
                ),
                (
                    "baby_assets",
                    Nbt::Compound(vec![
                        ("angry", Nbt::Str("minecraft:entity/wolf/wolf_woods_angry_baby")),
                        ("tame", Nbt::Str("minecraft:entity/wolf/wolf_woods_tame_baby")),
                        ("wild", Nbt::Str("minecraft:entity/wolf/wolf_woods_baby")),
                    ]),
                ),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![("type", Nbt::Str("minecraft:biome")), ("biomes", Nbt::Str("minecraft:forest"))]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn wolf_sound_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:angry",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_angry.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_angry.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_angry.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_angry.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_angry.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_angry.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:big",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_big.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_big.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_big.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_big.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_big.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_big.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:classic",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:cute",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_cute.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_cute.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_cute.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_cute.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_cute.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_cute.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:grumpy",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_grumpy.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_grumpy.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_grumpy.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_grumpy.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_grumpy.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_grumpy.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:puglin",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_puglin.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_puglin.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_puglin.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_puglin.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_puglin.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_puglin.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
        (
            "minecraft:sad",
            Nbt::Compound(vec![
                (
                    "adult_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.wolf_sad.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.wolf_sad.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.wolf_sad.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.wolf_sad.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.wolf_sad.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.wolf_sad.whine")),
                    ]),
                ),
                (
                    "baby_sounds",
                    Nbt::Compound(vec![
                        ("ambient_sound", Nbt::Str("minecraft:entity.baby_wolf.ambient")),
                        ("death_sound", Nbt::Str("minecraft:entity.baby_wolf.death")),
                        ("growl_sound", Nbt::Str("minecraft:entity.baby_wolf.growl")),
                        ("hurt_sound", Nbt::Str("minecraft:entity.baby_wolf.hurt")),
                        ("pant_sound", Nbt::Str("minecraft:entity.baby_wolf.pant")),
                        ("step_sound", Nbt::Str("minecraft:entity.baby_wolf.step")),
                        ("whine_sound", Nbt::Str("minecraft:entity.baby_wolf.whine")),
                    ]),
                ),
            ]),
        ),
    ]
}

pub fn world_clock() -> Entries<'static> {
    vec![("minecraft:overworld", Nbt::Compound(vec![])), ("minecraft:the_end", Nbt::Compound(vec![]))]
}

pub fn worldgen_biome() -> Entries<'static> {
    vec![(
        "minecraft:plains",
        Nbt::Compound(vec![
            (
                "attributes",
                Nbt::Compound(vec![
                    (
                        "minecraft:gameplay/natural_mob_spawns",
                        Nbt::Compound(vec![
                            (
                                "argument",
                                Nbt::Compound(vec![
                                    ("spawn_costs", Nbt::Compound(vec![])),
                                    (
                                        "spawns_by_category",
                                        Nbt::Compound(vec![
                                            (
                                                "ambient",
                                                Nbt::List(vec![Nbt::Compound(vec![
                                                    ("type", Nbt::Str("minecraft:bat")),
                                                    ("count", Nbt::Int(8)),
                                                    ("weight", Nbt::Int(10)),
                                                ])]),
                                            ),
                                            (
                                                "creature",
                                                Nbt::List(vec![
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:sheep")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(12)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:pig")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(10)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:chicken")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(10)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:cow")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(8)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:horse")),
                                                        (
                                                            "count",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("minecraft:uniform")),
                                                                ("max_inclusive", Nbt::Int(6)),
                                                                ("min_inclusive", Nbt::Int(2)),
                                                            ]),
                                                        ),
                                                        ("weight", Nbt::Int(5)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:donkey")),
                                                        (
                                                            "count",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("minecraft:uniform")),
                                                                ("max_inclusive", Nbt::Int(3)),
                                                                ("min_inclusive", Nbt::Int(1)),
                                                            ]),
                                                        ),
                                                        ("weight", Nbt::Int(1)),
                                                    ]),
                                                ]),
                                            ),
                                            (
                                                "monster",
                                                Nbt::List(vec![
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:spider")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(100)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:zombie")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(90)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:zombie_villager")),
                                                        ("count", Nbt::Int(1)),
                                                        ("weight", Nbt::Int(5)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:zombie_horse")),
                                                        ("count", Nbt::Int(1)),
                                                        ("weight", Nbt::Int(5)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:skeleton")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(100)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:creeper")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(100)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:slime")),
                                                        ("count", Nbt::Int(4)),
                                                        ("weight", Nbt::Int(100)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:enderman")),
                                                        (
                                                            "count",
                                                            Nbt::Compound(vec![
                                                                ("type", Nbt::Str("minecraft:uniform")),
                                                                ("max_inclusive", Nbt::Int(4)),
                                                                ("min_inclusive", Nbt::Int(1)),
                                                            ]),
                                                        ),
                                                        ("weight", Nbt::Int(10)),
                                                    ]),
                                                    Nbt::Compound(vec![
                                                        ("type", Nbt::Str("minecraft:witch")),
                                                        ("count", Nbt::Int(1)),
                                                        ("weight", Nbt::Int(5)),
                                                    ]),
                                                ]),
                                            ),
                                            (
                                                "underground_water_creature",
                                                Nbt::List(vec![Nbt::Compound(vec![
                                                    ("type", Nbt::Str("minecraft:glow_squid")),
                                                    (
                                                        "count",
                                                        Nbt::Compound(vec![
                                                            ("type", Nbt::Str("minecraft:uniform")),
                                                            ("max_inclusive", Nbt::Int(6)),
                                                            ("min_inclusive", Nbt::Int(4)),
                                                        ]),
                                                    ),
                                                    ("weight", Nbt::Int(10)),
                                                ])]),
                                            ),
                                        ]),
                                    ),
                                ]),
                            ),
                            ("modifier", Nbt::Str("overlay")),
                        ]),
                    ),
                    ("minecraft:visual/sky_color", Nbt::Int(7907327)),
                ]),
            ),
            (
                "carvers",
                Nbt::List(vec![
                    Nbt::Str("minecraft:cave"),
                    Nbt::Str("minecraft:cave_extra_underground"),
                    Nbt::Str("minecraft:canyon"),
                ]),
            ),
            ("downfall", Nbt::Float(0.4_f32)),
            ("effects", Nbt::Compound(vec![("water_color", Nbt::Int(4159204))])),
            (
                "features",
                Nbt::List(vec![
                    Nbt::List(vec![]),
                    Nbt::List(vec![Nbt::Str("minecraft:lake_lava_underground"), Nbt::Str("minecraft:lake_lava_surface")]),
                    Nbt::List(vec![Nbt::Str("minecraft:amethyst_geode")]),
                    Nbt::List(vec![Nbt::Str("minecraft:monster_room"), Nbt::Str("minecraft:monster_room_deep")]),
                    Nbt::List(vec![]),
                    Nbt::List(vec![]),
                    Nbt::List(vec![
                        Nbt::Str("minecraft:ore_dirt"),
                        Nbt::Str("minecraft:ore_gravel"),
                        Nbt::Str("minecraft:ore_granite_upper"),
                        Nbt::Str("minecraft:ore_granite_lower"),
                        Nbt::Str("minecraft:ore_diorite_upper"),
                        Nbt::Str("minecraft:ore_diorite_lower"),
                        Nbt::Str("minecraft:ore_andesite_upper"),
                        Nbt::Str("minecraft:ore_andesite_lower"),
                        Nbt::Str("minecraft:ore_tuff"),
                        Nbt::Str("minecraft:ore_coal_upper"),
                        Nbt::Str("minecraft:ore_coal_lower"),
                        Nbt::Str("minecraft:ore_iron_upper"),
                        Nbt::Str("minecraft:ore_iron_middle"),
                        Nbt::Str("minecraft:ore_iron_small"),
                        Nbt::Str("minecraft:ore_gold"),
                        Nbt::Str("minecraft:ore_gold_lower"),
                        Nbt::Str("minecraft:ore_redstone"),
                        Nbt::Str("minecraft:ore_redstone_lower"),
                        Nbt::Str("minecraft:ore_diamond"),
                        Nbt::Str("minecraft:ore_diamond_medium"),
                        Nbt::Str("minecraft:ore_diamond_large"),
                        Nbt::Str("minecraft:ore_diamond_buried"),
                        Nbt::Str("minecraft:ore_lapis"),
                        Nbt::Str("minecraft:ore_lapis_buried"),
                        Nbt::Str("minecraft:ore_copper"),
                        Nbt::Str("minecraft:underwater_magma"),
                        Nbt::Str("minecraft:disk_sand"),
                        Nbt::Str("minecraft:disk_clay"),
                        Nbt::Str("minecraft:disk_gravel"),
                    ]),
                    Nbt::List(vec![]),
                    Nbt::List(vec![Nbt::Str("minecraft:spring_water"), Nbt::Str("minecraft:spring_lava")]),
                    Nbt::List(vec![
                        Nbt::Str("minecraft:glow_lichen"),
                        Nbt::Str("minecraft:patch_tall_grass_2"),
                        Nbt::Str("minecraft:patch_bush"),
                        Nbt::Str("minecraft:trees_plains"),
                        Nbt::Str("minecraft:flower_plains"),
                        Nbt::Str("minecraft:patch_grass_plain"),
                        Nbt::Str("minecraft:brown_mushroom_normal"),
                        Nbt::Str("minecraft:red_mushroom_normal"),
                        Nbt::Str("minecraft:patch_pumpkin"),
                        Nbt::Str("minecraft:patch_sugar_cane"),
                        Nbt::Str("minecraft:patch_firefly_bush_near_water"),
                    ]),
                    Nbt::List(vec![Nbt::Str("minecraft:freeze_top_layer")]),
                ]),
            ),
            ("has_precipitation", Nbt::Bool(true)),
            ("temperature", Nbt::Float(0.8_f32)),
        ]),
    )]
}

pub fn zombie_nautilus_variant() -> Entries<'static> {
    vec![
        (
            "minecraft:temperate",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/nautilus/zombie_nautilus")),
                ("spawn_conditions", Nbt::List(vec![Nbt::Compound(vec![("priority", Nbt::Int(0))])])),
            ]),
        ),
        (
            "minecraft:warm",
            Nbt::Compound(vec![
                ("asset_id", Nbt::Str("minecraft:entity/nautilus/zombie_nautilus_coral")),
                ("model", Nbt::Str("warm")),
                (
                    "spawn_conditions",
                    Nbt::List(vec![Nbt::Compound(vec![
                        (
                            "condition",
                            Nbt::Compound(vec![
                                ("type", Nbt::Str("minecraft:biome")),
                                ("biomes", Nbt::Str("#minecraft:spawns_coral_variant_zombie_nautilus")),
                            ]),
                        ),
                        ("priority", Nbt::Int(1)),
                    ])]),
                ),
            ]),
        ),
    ]
}

pub fn banner_pattern_tags() -> Tags {
    vec![
        (
            "minecraft:no_item_required",
            vec![26, 27, 28, 29, 31, 38, 35, 37, 32, 36, 34, 33, 25, 5, 30, 39, 40, 41, 42, 7, 10, 9, 8, 3, 23, 19, 17, 20, 18, 1, 14, 15],
        ),
        ("minecraft:pattern_item/bordure_indented", vec![6]),
        ("minecraft:pattern_item/creeper", vec![4]),
        ("minecraft:pattern_item/field_masoned", vec![2]),
        ("minecraft:pattern_item/flow", vec![11]),
        ("minecraft:pattern_item/flower", vec![12]),
        ("minecraft:pattern_item/globe", vec![13]),
        ("minecraft:pattern_item/guster", vec![16]),
        ("minecraft:pattern_item/mojang", vec![21]),
        ("minecraft:pattern_item/piglin", vec![22]),
        ("minecraft:pattern_item/skull", vec![24]),
    ]
}

pub fn damage_type_tags() -> Tags {
    vec![
        ("minecraft:always_hurts_ender_dragons", vec![15, 9, 35, 1]),
        ("minecraft:always_kills_armor_stands", vec![0, 46, 14, 50, 48]),
        ("minecraft:always_most_significant_fall", vec![32]),
        ("minecraft:always_triggers_silverfish", vec![27]),
        ("minecraft:avoids_guardian_thorns", vec![27, 44, 15, 9, 35, 1]),
        ("minecraft:burn_from_stepping", vec![3, 20, 42]),
        ("minecraft:burns_armor_stands", vec![31]),
        (
            "minecraft:bypasses_armor",
            vec![31, 22, 4, 6, 16, 18, 49, 5, 40, 10, 8, 17, 39, 27, 23, 32, 19, 36, 33],
        ),
        ("minecraft:bypasses_cooldown", vec![]),
        ("minecraft:bypasses_effects", vec![40]),
        ("minecraft:bypasses_enchantments", vec![36]),
        ("minecraft:bypasses_invulnerability", vec![32, 19]),
        ("minecraft:bypasses_resistance", vec![32, 19]),
        (
            "minecraft:bypasses_shield",
            vec![31, 22, 4, 6, 16, 18, 49, 5, 40, 10, 8, 17, 39, 27, 23, 32, 19, 36, 33, 2, 3, 7, 11, 13, 20, 42, 21, 24, 25, 43],
        ),
        ("minecraft:bypasses_wolf_armor", vec![32, 19, 4, 6, 7, 17, 22, 23, 27, 33, 40, 44, 49]),
        ("minecraft:can_break_armor_stand", vec![35, 34, 37, 26]),
        ("minecraft:damages_helmet", vec![11, 12, 13]),
        ("minecraft:ignites_armor_stands", vec![21, 3]),
        ("minecraft:is_drowning", vec![6]),
        ("minecraft:is_explosion", vec![15, 9, 35, 1]),
        ("minecraft:is_fall", vec![10, 8, 39]),
        ("minecraft:is_fire", vec![21, 3, 31, 24, 20, 42, 47, 14]),
        ("minecraft:is_freezing", vec![17]),
        ("minecraft:is_lightning", vec![25]),
        ("minecraft:is_player_attack", vec![34, 37, 26]),
        ("minecraft:is_projectile", vec![0, 46, 30, 47, 14, 50, 45, 48]),
        ("minecraft:mace_smash", vec![26]),
        ("minecraft:no_anger", vec![29]),
        ("minecraft:no_impact", vec![6]),
        (
            "minecraft:no_knockback",
            vec![9, 35, 1, 21, 25, 31, 24, 20, 42, 22, 4, 6, 40, 2, 10, 8, 16, 32, 18, 27, 49, 5, 7, 43, 17, 39, 33, 19, 3, 37],
        ),
        ("minecraft:no_wolf_retaliation", vec![42]),
        (
            "minecraft:panic_causes",
            vec![2, 17, 20, 42, 21, 24, 25, 31, 0, 5, 9, 14, 15, 23, 27, 28, 30, 35, 36, 41, 45, 46, 47, 48, 49, 50, 34, 37, 26],
        ),
        ("minecraft:panic_environmental_causes", vec![2, 17, 20, 42, 21, 24, 25, 31]),
        (
            "minecraft:sulfur_cube_with_block_immune_to",
            vec![0, 2, 7, 10, 11, 12, 13, 17, 26, 20, 28, 29, 30, 34, 37, 38, 39, 41, 42, 43, 45, 46, 48, 15, 9, 35, 1],
        ),
        ("minecraft:witch_resistant_to", vec![27, 23, 36, 44]),
        ("minecraft:wither_immune_to", vec![6]),
    ]
}

pub fn instrument_tags() -> Tags {
    vec![
        ("minecraft:goat_horns", vec![4, 6, 5, 3, 0, 1, 7, 2]),
        ("minecraft:regular_goat_horns", vec![4, 6, 5, 3]),
        ("minecraft:screaming_goat_horns", vec![0, 1, 7, 2]),
    ]
}

pub fn timeline_tags() -> Tags {
    vec![
        ("minecraft:in_end", vec![3]),
        ("minecraft:in_nether", vec![3]),
        ("minecraft:in_overworld", vec![3, 0, 2, 1]),
        ("minecraft:universal", vec![3]),
    ]
}

/// All synchronized registries: (registry id, entries).
pub fn all() -> Vec<(&'static str, Entries<'static>)> {
    vec![
        ("minecraft:banner_pattern", banner_pattern()),
        ("minecraft:block_transformer", block_transformer()),
        ("minecraft:cat_variant", cat_variant()),
        ("minecraft:cat_sound_variant", cat_sound_variant()),
        ("minecraft:chicken_variant", chicken_variant()),
        ("minecraft:chicken_sound_variant", chicken_sound_variant()),
        ("minecraft:cow_variant", cow_variant()),
        ("minecraft:cow_sound_variant", cow_sound_variant()),
        ("minecraft:damage_type", damage_type()),
        ("minecraft:dimension_type", dimension_type()),
        ("minecraft:frog_variant", frog_variant()),
        ("minecraft:instrument", instrument()),
        ("minecraft:jukebox_song", jukebox_song()),
        ("minecraft:painting_variant", painting_variant()),
        ("minecraft:decorated_pot_pattern", decorated_pot_pattern()),
        ("minecraft:pig_variant", pig_variant()),
        ("minecraft:pig_sound_variant", pig_sound_variant()),
        ("minecraft:sulfur_cube_archetype", sulfur_cube_archetype()),
        ("minecraft:timeline", timeline()),
        ("minecraft:trim_material", trim_material()),
        ("minecraft:wolf_variant", wolf_variant()),
        ("minecraft:wolf_sound_variant", wolf_sound_variant()),
        ("minecraft:world_clock", world_clock()),
        ("minecraft:worldgen/biome", worldgen_biome()),
        ("minecraft:zombie_nautilus_variant", zombie_nautilus_variant()),
    ]
}

/// All sent tags: (registry id, (tag, entry ids)).
pub fn all_tags() -> Vec<(&'static str, Tags)> {
    vec![
        ("minecraft:banner_pattern", banner_pattern_tags()),
        ("minecraft:damage_type", damage_type_tags()),
        ("minecraft:instrument", instrument_tags()),
        ("minecraft:timeline", timeline_tags()),
    ]
}

/// Tags in built-in registries (numeric IDs unknown here) that network
/// data references. Sent empty: they parse and mean nothing matches.
pub fn builtin_empty_tags() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "minecraft:block",
            vec!["minecraft:infiniburn_end", "minecraft:infiniburn_nether", "minecraft:infiniburn_overworld"],
        ),
        (
            "minecraft:item",
            vec![
                "minecraft:sulfur_cube_archetype/bouncy",
                "minecraft:sulfur_cube_archetype/explosive",
                "minecraft:sulfur_cube_archetype/fast_flat",
                "minecraft:sulfur_cube_archetype/fast_sliding",
                "minecraft:sulfur_cube_archetype/high_resistance",
                "minecraft:sulfur_cube_archetype/hot",
                "minecraft:sulfur_cube_archetype/light",
                "minecraft:sulfur_cube_archetype/regular",
                "minecraft:sulfur_cube_archetype/slow_bouncy",
                "minecraft:sulfur_cube_archetype/slow_flat",
                "minecraft:sulfur_cube_archetype/slow_sliding",
                "minecraft:sulfur_cube_archetype/sticky",
            ],
        ),
    ]
}
