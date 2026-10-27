use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Cdlc {
    GlobalMobilization,
    SogPrairieFire,
    CslaIronCurtain,
    WesternSahara,
    Spearhead1944,
    ReactionForces,
    ExpeditionaryForces,
}

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

    pub const fn short_form(self) -> &'static str {
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
