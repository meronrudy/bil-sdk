use ratatui::style::Color;
use serde::{Deserialize, Deserializer, Serializer};

pub const C_BG: Color = Color::Rgb(13, 17, 23);
pub const C_BG2: Color = Color::Rgb(22, 27, 34);
pub const C_BG3: Color = Color::Rgb(33, 38, 45);
pub const C_BORDER: Color = Color::Rgb(48, 54, 61);
pub const C_TEXT: Color = Color::Rgb(230, 237, 243);
pub const C_MUTED: Color = Color::Rgb(139, 148, 158);
pub const C_DIM: Color = Color::Rgb(72, 79, 88);
pub const C_CYAN: Color = Color::Rgb(57, 212, 201);
pub const C_AMBER: Color = Color::Rgb(240, 165, 0);
pub const C_RED: Color = Color::Rgb(248, 81, 73);
pub const C_GREEN: Color = Color::Rgb(63, 185, 80);
pub const C_BLUE: Color = Color::Rgb(88, 166, 255);
pub const C_PURPLE: Color = Color::Rgb(188, 140, 255);

pub mod color_serde {
    use super::*;

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_color(*color))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        decode_color(&value).map_err(serde::de::Error::custom)
    }

    fn encode_color(color: Color) -> String {
        match color {
            Color::Reset => "reset".to_string(),
            Color::Black => "black".to_string(),
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Magenta => "magenta".to_string(),
            Color::Cyan => "cyan".to_string(),
            Color::Gray => "gray".to_string(),
            Color::DarkGray => "dark-gray".to_string(),
            Color::LightRed => "light-red".to_string(),
            Color::LightGreen => "light-green".to_string(),
            Color::LightYellow => "light-yellow".to_string(),
            Color::LightBlue => "light-blue".to_string(),
            Color::LightMagenta => "light-magenta".to_string(),
            Color::LightCyan => "light-cyan".to_string(),
            Color::White => "white".to_string(),
            Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
            Color::Indexed(index) => format!("indexed:{index}"),
        }
    }

    fn decode_color(value: &str) -> Result<Color, String> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "reset" => Ok(Color::Reset),
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "gray" => Ok(Color::Gray),
            "dark-gray" => Ok(Color::DarkGray),
            "light-red" => Ok(Color::LightRed),
            "light-green" => Ok(Color::LightGreen),
            "light-yellow" => Ok(Color::LightYellow),
            "light-blue" => Ok(Color::LightBlue),
            "light-magenta" => Ok(Color::LightMagenta),
            "light-cyan" => Ok(Color::LightCyan),
            "white" => Ok(Color::White),
            _ if normalized.starts_with("indexed:") => normalized["indexed:".len()..]
                .parse::<u8>()
                .map(Color::Indexed)
                .map_err(|err| format!("invalid indexed color '{value}': {err}")),
            _ if normalized.starts_with('#') && normalized.len() == 7 => {
                let red = u8::from_str_radix(&normalized[1..3], 16)
                    .map_err(|err| format!("invalid red component in '{value}': {err}"))?;
                let green = u8::from_str_radix(&normalized[3..5], 16)
                    .map_err(|err| format!("invalid green component in '{value}': {err}"))?;
                let blue = u8::from_str_radix(&normalized[5..7], 16)
                    .map_err(|err| format!("invalid blue component in '{value}': {err}"))?;
                Ok(Color::Rgb(red, green, blue))
            }
            _ => Err(format!("unsupported color value '{value}'")),
        }
    }
}
