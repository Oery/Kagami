//! Map items and locator bar waypoints for protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

/// A marker on a map, such as a player or a banner.
#[derive(Debug, IntoOwned)]
pub struct MapIcon<'p> {
    /// ID in the `minecraft:map_decoration_type` registry.
    pub kind: i32,
    /// -128 is the left edge, 127 the right one.
    pub x: i8,
    /// -128 is the top edge, 127 the bottom one.
    pub z: i8,
    /// Sixteenths of a turn, 0 to 15.
    pub direction: i8,
    pub name: Option<Nbt<'p>>,
}

impl<'p> PacketWriter<'p> for MapIcon<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let kind = VarInt::read(input)?.0;
        let x = i8::read(input)?;
        let z = i8::read(input)?;
        let direction = i8::read(input)?;
        let name = Option::read(input)?;

        Ok(Self { kind, x, z, direction, name })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.kind).encode(out)?;
        self.x.encode(out)?;
        self.z.encode(out)?;
        self.direction.encode(out)?;

        self.name.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.kind).body_len() + 3 + self.name.body_len()
    }
}

/// A rectangle of map colors, row by row.
#[derive(Debug, IntoOwned)]
pub struct MapPatch<'p> {
    /// Never zero: zero columns means no patch.
    pub columns: u8,
    pub rows: u8,
    pub x: u8,
    pub z: u8,
    pub colors: Cow<'p, [u8]>,
}

/// A map's colors are unchanged, or updated by one patch. The wire has no
/// presence flag: a zero column count stands for no patch.
#[derive(Debug, IntoOwned)]
pub enum MapColors<'p> {
    Unchanged,
    Patch(MapPatch<'p>),
}

impl<'p> PacketWriter<'p> for MapColors<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let columns = u8::read(input)?;
        if columns == 0 {
            return Ok(Self::Unchanged);
        }

        let rows = u8::read(input)?;
        let x = u8::read(input)?;
        let z = u8::read(input)?;
        let colors = Cow::read(input)?;

        Ok(Self::Patch(MapPatch { columns, rows, x, z, colors }))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let Self::Patch(patch) = self else {
            return 0u8.encode(out);
        };

        patch.columns.encode(out)?;
        patch.rows.encode(out)?;
        patch.x.encode(out)?;
        patch.z.encode(out)?;

        patch.colors.encode(out)
    }

    fn body_len(&self) -> usize {
        match self {
            Self::Unchanged => 1,
            Self::Patch(patch) => 4 + patch.colors.body_len(),
        }
    }
}

#[packet(Play, 0x34, Clientbound)]
pub struct MapData<'p> {
    #[format = "varint"]
    pub map_id: i32,
    /// 0 is one block per pixel, 4 is sixteen.
    pub scale: i8,
    pub locked: bool,
    /// `None` keeps the current icons.
    pub icons: Option<Vec<MapIcon<'p>>>,
    pub colors: MapColors<'p>,
}

/// `Waypoint` operations.
pub mod waypoint_operation {
    pub const TRACK: i32 = 0;
    pub const UNTRACK: i32 = 1;
    pub const UPDATE: i32 = 2;
}

/// What a waypoint is known by: usually its entity's UUID.
#[derive(Debug, IntoOwned)]
pub enum WaypointId<'p> {
    Uuid(Uuid),
    Name(Cow<'p, str>),
}

impl<'p> PacketWriter<'p> for WaypointId<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let is_uuid = bool::read(input)?;
        if is_uuid {
            return Ok(Self::Uuid(Uuid::read(input)?));
        }

        Ok(Self::Name(Cow::read(input)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Uuid(uuid) => {
                true.encode(out)?;
                uuid.encode(out)
            }
            Self::Name(name) => {
                false.encode(out)?;
                name.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        let id = match self {
            Self::Uuid(_) => 16,
            Self::Name(name) => name.body_len(),
        };

        1 + id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoOwned)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PacketWriter<'_> for Rgb {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let red = u8::read(input)?;
        let green = u8::read(input)?;
        let blue = u8::read(input)?;

        Ok(Self { red, green, blue })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        out.extend_from_slice(&[self.red, self.green, self.blue]);

        Ok(())
    }

    fn body_len(&self) -> usize {
        3
    }
}

/// Where a waypoint points to.
#[derive(Clone, Copy, Debug, PartialEq, IntoOwned)]
pub enum WaypointTarget {
    None,
    Block {
        x: i32,
        y: i32,
        z: i32,
    },
    Chunk {
        x: i32,
        z: i32,
    },
    /// Radians.
    Azimuth(f32),
}

impl PacketWriter<'_> for WaypointTarget {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let target = match VarInt::read(input)?.0 {
            0 => Self::None,
            1 => {
                let x = VarInt::read(input)?.0;
                let y = VarInt::read(input)?.0;
                let z = VarInt::read(input)?.0;
                Self::Block { x, y, z }
            }
            2 => {
                let x = VarInt::read(input)?.0;
                let z = VarInt::read(input)?.0;
                Self::Chunk { x, z }
            }
            3 => Self::Azimuth(f32::read(input)?),
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        Ok(target)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match *self {
            Self::None => VarInt(0).encode(out),
            Self::Block { x, y, z } => {
                VarInt(1).encode(out)?;
                VarInt(x).encode(out)?;
                VarInt(y).encode(out)?;
                VarInt(z).encode(out)
            }
            Self::Chunk { x, z } => {
                VarInt(2).encode(out)?;
                VarInt(x).encode(out)?;
                VarInt(z).encode(out)
            }
            Self::Azimuth(angle) => {
                VarInt(3).encode(out)?;
                angle.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        let data = match *self {
            Self::None => 0,
            Self::Block { x, y, z } => VarInt(x).body_len() + VarInt(y).body_len() + VarInt(z).body_len(),
            Self::Chunk { x, z } => VarInt(x).body_len() + VarInt(z).body_len(),
            Self::Azimuth(_) => 4,
        };

        1 + data
    }
}

/// Tracks, untracks or updates an entry of the locator bar.
#[packet(Play, 0x8D, Clientbound)]
pub struct Waypoint<'p> {
    /// A [`waypoint_operation`] value.
    #[format = "varint"]
    pub operation: i32,
    pub id: WaypointId<'p>,
    /// A `waypoint_style` asset, by identifier.
    pub style: Cow<'p, str>,
    pub color: Option<Rgb>,
    pub target: WaypointTarget,
}
