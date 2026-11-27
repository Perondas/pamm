use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, Hash)]
pub enum Cdlc {
    GlobalMobilization,
    SogPrairieFire,
    CslaIronCurtain,
    WesternSahara,
    Spearhead1944,
    ReactionForces,
    ExpeditionaryForces,
}

const CDLC_COMPAT_MOD_IDS: [&str; 7] = [
    "1776428269", // Global Mobilization
    "2477276806", // SOG Prairie Fire
    "2503886780", // CSLA Iron Curtain
    "2636962953", // Western Sahara
    "2991828484", // Spearhead 1944
    "3150497912", // Reaction Forces
    "3348605126", // Expeditionary Forces
];

impl Cdlc {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::GlobalMobilization => "Global Mobilization",
            Self::SogPrairieFire => "SOG Prairie Fire",
            Self::CslaIronCurtain => "CSLA Iron Curtain",
            Self::WesternSahara => "Western Sahara",
            Self::Spearhead1944 => "Spearhead 1944",
            Self::ReactionForces => "Reaction Forces",
            Self::ExpeditionaryForces => "Expeditionary Forces",
        }
    }

    pub const fn short_form(&self) -> &'static str {
        match self {
            Self::GlobalMobilization => "gm",
            Self::SogPrairieFire => "vn",
            Self::CslaIronCurtain => "csla",
            Self::WesternSahara => "ws",
            Self::Spearhead1944 => "spe",
            Self::ReactionForces => "rf",
            Self::ExpeditionaryForces => "ef",
        }
    }

    pub const fn compat_mod_id(&self) -> &'static str {
        match self {
            Cdlc::GlobalMobilization => "1776428269",
            Cdlc::SogPrairieFire => "2477276806",
            Cdlc::CslaIronCurtain => "2503886780",
            Cdlc::WesternSahara => "2636962953",
            Cdlc::Spearhead1944 => "2991828484",
            Cdlc::ReactionForces => "3150497912",
            Cdlc::ExpeditionaryForces => "3348605126",
        }
    }

    pub fn from_short_form(short_form: &str) -> Self {
        match short_form {
            "gm" => Self::GlobalMobilization,
            "vn" => Self::SogPrairieFire,
            "csla" => Self::CslaIronCurtain,
            "ws" => Self::WesternSahara,
            "spe" => Self::Spearhead1944,
            "rf" => Self::ReactionForces,
            "ef" => Self::ExpeditionaryForces,
            _ => panic!("Invalid short form: {}", short_form),
        }
    }
}

pub fn is_compat_mod(id: &str) -> bool {
    CDLC_COMPAT_MOD_IDS.contains(&id)
}

#[cfg(test)]
mod tests {
    use super::Cdlc;

    #[test]
    fn exposes_names_and_short_forms() {
        let expected = [
            (Cdlc::GlobalMobilization, "Global Mobilization", "gm"),
            (Cdlc::SogPrairieFire, "SOG Prairie Fire", "vn"),
            (Cdlc::CslaIronCurtain, "CSLA Iron Curtain", "csla"),
            (Cdlc::WesternSahara, "Western Sahara", "ws"),
            (Cdlc::Spearhead1944, "Spearhead 1944", "spe"),
            (Cdlc::ReactionForces, "Reaction Forces", "rf"),
            (Cdlc::ExpeditionaryForces, "Expeditionary Forces", "ef"),
        ];

        for (cdlc, name, short_form) in expected {
            assert_eq!(cdlc.name(), name);
            assert_eq!(cdlc.short_form(), short_form);
        }
    }
}
