use crate::{
    McPacket,
    codec::{
        identifier::Identifier, prefixed_array::PrefixedArray,
        var_int::VarInt,
    },
    messages::models::{slot::Slot, text_component::TextComponent},
};
use mc_packet_macros::mc_packet;
use serde::{
    Deserialize,
    de::{self, Visitor},
};

/// Clientbound update advancements packet during play phase.
#[derive(Debug, Deserialize)]
#[mc_packet(0x80)]
pub struct UpdateAdvancements {
    /// Whether to reset/clear the current advancements.
    reset_clear: bool,
    /// The mapping of advancement IDs to their advancement data. The advancement data is not sent in this packet, but is instead sent in the advancement mapping field.
    advancement_mapping: PrefixedArray<AdvancementMapping>,
    /// The identifiers of the advancements that should be removed.
    identifiers: PrefixedArray<Identifier>,
    /// The mapping of advancement IDs to their advancement progress.
    progress_mapping: PrefixedArray<AdvancementProgressMapping>,
    /// Whether to show the advancements screen if there are new advancements.
    show_advancements: bool,
}

/// A mapping of an advancement ID to its advancement data.
#[derive(Debug, Deserialize)]
pub struct AdvancementMapping {
    /// The identifier of the advancement.
    key: Identifier,
    /// The advancement data for the advancement.
    value: Advancement,
}

/// Advancement data.
#[derive(Debug, Deserialize)]
pub struct Advancement {
    /// The identifier of the parent advancement.
    parent_id: Option<Identifier>,
    /// Display data for the advancement.
    display_data: Option<AdvancementDisplay>,
    /// Array with a sub-array of criteria. To check if the requirements are met, each sub-array must be tested and mapped with the OR operator, resulting in a boolean array.
    /// These booleans must be mapped with the AND operator to get the result.\
    /// Strings have a max-size of 32767 characters.
    nested_requirements: PrefixedArray<PrefixedArray<String>>,
    /// Whether the client should include this achievement in the telemetry data when it's completed.
    /// The vanilla client only sends data for advancements on the minecraft namespace.
    sends_telemetry_data: bool,
}

/// How the advancement is displayed.
#[derive(Debug)]
pub struct AdvancementDisplay {
    /// The title of the advancement.
    title: TextComponent,
    /// The description of the advancement.
    description: TextComponent,
    /// The icon to show for the advancement.
    icon: Slot,
    /// The frame type of the advancement.
    frame_type: AdvancementFrameType,
    /// Flags for the advancement display.
    flags: AdvancementDisplayFlags,
    /// The background texture to show behind the advancement, only present if the flags indicate so.
    background_texture: Option<Identifier>,
    /// X coordinate in the UI.
    x_coord: f32,
    /// Y coordinate in the UI.
    y_coord: f32,
}

struct AdvancementDisplayVisitor;

impl<'de> Visitor<'de> for AdvancementDisplayVisitor {
    type Value = AdvancementDisplay;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of title, description, icon, frame type, flags, and optionally background texture")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let title = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(0, &self))?;

        let description = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(1, &self))?;

        let icon = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(2, &self))?;

        let frame_type = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(3, &self))?;

        let flags: AdvancementDisplayFlags = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(4, &self))?;

        let background_texture = if flags.has_background_texture {
            Some(
                seq.next_element()?
                    .ok_or(de::Error::invalid_length(5, &self))?,
            )
        } else {
            None
        };

        let x_coord = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(6, &self))?;

        let y_coord = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(7, &self))?;

        Ok(AdvancementDisplay {
            title,
            description,
            icon,
            frame_type,
            flags,
            background_texture,
            x_coord,
            y_coord,
        })
    }
}

impl<'de> Deserialize<'de> for AdvancementDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(AdvancementDisplayVisitor)
    }
}

/// The type of frame an advancement is.
#[derive(Debug)]
pub enum AdvancementFrameType {
    /// A task advancement.
    Task,
    /// A goal advancement.
    Goal,
    /// A challenge advancement.
    Challenge,
}

impl<'de> Deserialize<'de> for AdvancementFrameType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let frame_type: VarInt = VarInt::deserialize(deserializer)?;

        match frame_type.0 {
            0 => Ok(Self::Task),
            1 => Ok(Self::Challenge),
            2 => Ok(Self::Goal),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid advancement frame type: {}",
                frame_type.0
            ))),
        }
    }
}

/// Flags for how the advancement is displayed.
#[derive(Debug)]
pub struct AdvancementDisplayFlags {
    /// Whether the advancement has a background texture.
    pub has_background_texture: bool,
    /// Whether to show a toast notification when the advancement is completed.
    pub show_toast: bool,
    /// Whether the advancement is hidden.
    pub is_hidden: bool,
}

impl<'de> Deserialize<'de> for AdvancementDisplayFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = i32::deserialize(deserializer)?;

        Ok(Self {
            has_background_texture: flags & 0x1 != 0,
            show_toast: flags & 0x2 != 0,
            is_hidden: flags & 0x4 != 0,
        })
    }
}

/// A mapping of an advancement ID to its advancement progress.
#[derive(Debug, Deserialize)]
pub struct AdvancementProgressMapping {
    /// The identifier of the advancement.
    pub key: Identifier,
    /// The advancement progress for the advancement.
    pub value: AdvancementProgress,
}

/// Advancement progress for an advancement, such as which criteria have been completed and when.
#[derive(Debug, Deserialize)]
pub struct AdvancementProgress {
    /// Mapping of criterion identifier to criterion progress.
    criteria: PrefixedArray<CriterionProgress>,
}

/// Criterion progress for a single criterion.
#[derive(Debug, Deserialize)]
pub struct CriterionProgress {
    /// The identifier of the criterion.
    criterion_identifier: Identifier,
    /// Present if achieved. As returned by Date.getTime.
    // TODO: It is a long but it should really be a date type.
    date_of_achieving: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::UpdateAdvancements;
    use crate::ser::deserializer::Deserializer;
    use serde::Deserialize;
    use std::io::Cursor;

    #[test]
    fn deserializes_known_update_advancements_payload() {
        let data: Vec<u8> = vec![
            0x01, 0x02, 0x2c, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x3a, 0x72,
            0x65, 0x63, 0x69, 0x70, 0x65, 0x73, 0x2f, 0x64, 0x65, 0x63, 0x6f, 0x72, 0x61, 0x74,
            0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x72, 0x61, 0x66, 0x74, 0x69, 0x6e, 0x67, 0x5f,
            0x74, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x16, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61,
            0x66, 0x74, 0x3a, 0x72, 0x65, 0x63, 0x69, 0x70, 0x65, 0x73, 0x2f, 0x72, 0x6f, 0x6f,
            0x74, 0x00, 0x01, 0x02, 0x0e, 0x68, 0x61, 0x73, 0x5f, 0x74, 0x68, 0x65, 0x5f, 0x72,
            0x65, 0x63, 0x69, 0x70, 0x65, 0x11, 0x75, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72,
            0x69, 0x67, 0x68, 0x74, 0x5f, 0x61, 0x77, 0x61, 0x79, 0x00, 0x16, 0x6d, 0x69, 0x6e,
            0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x3a, 0x72, 0x65, 0x63, 0x69, 0x70, 0x65, 0x73,
            0x2f, 0x72, 0x6f, 0x6f, 0x74, 0x00, 0x00, 0x01, 0x01, 0x0a, 0x69, 0x6d, 0x70, 0x6f,
            0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x00, 0x00, 0x02, 0x2c, 0x6d, 0x69, 0x6e, 0x65,
            0x63, 0x72, 0x61, 0x66, 0x74, 0x3a, 0x72, 0x65, 0x63, 0x69, 0x70, 0x65, 0x73, 0x2f,
            0x64, 0x65, 0x63, 0x6f, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x72,
            0x61, 0x66, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x02, 0x0e,
            0x68, 0x61, 0x73, 0x5f, 0x74, 0x68, 0x65, 0x5f, 0x72, 0x65, 0x63, 0x69, 0x70, 0x65,
            0x00, 0x11, 0x75, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72, 0x69, 0x67, 0x68, 0x74,
            0x5f, 0x61, 0x77, 0x61, 0x79, 0x01, 0x00, 0x00, 0x01, 0x9d, 0x3b, 0xba, 0x14, 0xc0,
            0x16, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x3a, 0x72, 0x65, 0x63,
            0x69, 0x70, 0x65, 0x73, 0x2f, 0x72, 0x6f, 0x6f, 0x74, 0x01, 0x0a, 0x69, 0x6d, 0x70,
            0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x00, 0x01,
        ];

        let mut de = Deserializer::new(Cursor::new(data));
        let parsed = UpdateAdvancements::deserialize(&mut de);
        assert!(
            parsed.is_ok(),
            "failed to parse update advancements: {parsed:?}"
        );
    }
}
