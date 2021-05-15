use crate::iso::{Iso2, Iso3};

pub struct Country {
    pub name: &'static str,
    pub iso2: Iso2,
    pub iso3: Iso3,
    // country_code (str)
    // calling code (nr)
    // date format (str)
    // capital (str)
}

pub const NUMBER_OF_COUNTRIES: u8 = 246;

impl Country {
    pub const ABW: Country = Country {
        name: "Aruba",
        iso2: Iso2::AW,
        iso3: Iso3::ABW,
    };
    pub const AFG: Country = Country {
        name: "Afghanistan",
        iso2: Iso2::AF,
        iso3: Iso3::AFG,
    };
    pub const AGO: Country = Country {
        name: "Angola",
        iso2: Iso2::AO,
        iso3: Iso3::AGO,
    };
    pub const AIA: Country = Country {
        name: "Anguilla",
        iso2: Iso2::AI,
        iso3: Iso3::AIA,
    };
    pub const ALA: Country = Country {
        name: "Åland Islands",
        iso2: Iso2::AX,
        iso3: Iso3::ALA,
    };
    pub const ALB: Country = Country {
        name: "Albania",
        iso2: Iso2::AL,
        iso3: Iso3::ALB,
    };
    pub const AND: Country = Country {
        name: "Andorra",
        iso2: Iso2::AD,
        iso3: Iso3::AND,
    };
    pub const ANT: Country = Country {
        name: "Netherlands Antilles",
        iso2: Iso2::AN,
        iso3: Iso3::ANT,
    };
    pub const ARE: Country = Country {
        name: "United Arab Emirates",
        iso2: Iso2::AE,
        iso3: Iso3::ARE,
    };
    pub const ARG: Country = Country {
        name: "Argentina",
        iso2: Iso2::AR,
        iso3: Iso3::ARG,
    };
    pub const ARM: Country = Country {
        name: "Armenia",
        iso2: Iso2::AM,
        iso3: Iso3::ARM,
    };
    pub const ASM: Country = Country {
        name: "American Samoa",
        iso2: Iso2::AS,
        iso3: Iso3::ASM,
    };
    pub const ATA: Country = Country {
        name: "Antarctica",
        iso2: Iso2::AQ,
        iso3: Iso3::ATA,
    };
    pub const ATF: Country = Country {
        name: "French Southern Territories",
        iso2: Iso2::TF,
        iso3: Iso3::ATF,
    };
    pub const ATG: Country = Country {
        name: "Antigua and Barbuda",
        iso2: Iso2::AG,
        iso3: Iso3::ATG,
    };
    pub const AUS: Country = Country {
        name: "Australia",
        iso2: Iso2::AU,
        iso3: Iso3::AUS,
    };
    pub const AUT: Country = Country {
        name: "Austria",
        iso2: Iso2::AT,
        iso3: Iso3::AUT,
    };
    pub const AZE: Country = Country {
        name: "Azerbaijan",
        iso2: Iso2::AZ,
        iso3: Iso3::AZE,
    };

    pub const BDI: Country = Country {
        name: "Burundi",
        iso2: Iso2::BI,
        iso3: Iso3::BDI,
    };
    pub const BEL: Country = Country {
        name: "Belgium",
        iso2: Iso2::BE,
        iso3: Iso3::BEL,
    };
    pub const BEN: Country = Country {
        name: "Benin",
        iso2: Iso2::BJ,
        iso3: Iso3::BEN,
    };
    pub const BFA: Country = Country {
        name: "Burkina Faso",
        iso2: Iso2::BF,
        iso3: Iso3::BFA,
    };
    pub const BGD: Country = Country {
        name: "Bangladesh",
        iso2: Iso2::BD,
        iso3: Iso3::BGD,
    };
    pub const BGR: Country = Country {
        name: "Bulgaria",
        iso2: Iso2::BG,
        iso3: Iso3::BGR,
    };
    pub const BHR: Country = Country {
        name: "Bahrain",
        iso2: Iso2::BH,
        iso3: Iso3::BHR,
    };
    pub const BHS: Country = Country {
        name: "Bahamas",
        iso2: Iso2::BS,
        iso3: Iso3::BHS,
    };
    pub const BIH: Country = Country {
        name: "Bosnia and Herzegovina",
        iso2: Iso2::BA,
        iso3: Iso3::BIH,
    };
    pub const BLM: Country = Country {
        name: "Saint Barthélemy",
        iso2: Iso2::BL,
        iso3: Iso3::BLM,
    };
    pub const BLR: Country = Country {
        name: "Belarus",
        iso2: Iso2::BY,
        iso3: Iso3::BLR,
    };
    pub const BLZ: Country = Country {
        name: "Belize",
        iso2: Iso2::BZ,
        iso3: Iso3::BLZ,
    };
    pub const BMU: Country = Country {
        name: "Bermuda",
        iso2: Iso2::BM,
        iso3: Iso3::BMU,
    };
    pub const BOL: Country = Country {
        name: "Bolivia",
        iso2: Iso2::BO,
        iso3: Iso3::BOL,
    };
    pub const BRA: Country = Country {
        name: "Brazil",
        iso2: Iso2::BR,
        iso3: Iso3::BRA,
    };
    pub const BRB: Country = Country {
        name: "Barbados",
        iso2: Iso2::BB,
        iso3: Iso3::BRB,
    };
    pub const BRN: Country = Country {
        name: "Brunei Darussalam",
        iso2: Iso2::BN,
        iso3: Iso3::BRN,
    };
    pub const BTN: Country = Country {
        name: "Bhutan",
        iso2: Iso2::BT,
        iso3: Iso3::BTN,
    };
    pub const BVT: Country = Country {
        name: "Bouvet Island",
        iso2: Iso2::BV,
        iso3: Iso3::BVT,
    };
    pub const BWA: Country = Country {
        name: "Botswana",
        iso2: Iso2::BW,
        iso3: Iso3::BWA,
    };

    pub const CAF: Country = Country {
        name: "Central African Republic",
        iso2: Iso2::CF,
        iso3: Iso3::CAF,
    };
    pub const CAN: Country = Country {
        name: "Canada",
        iso2: Iso2::CA,
        iso3: Iso3::CAN,
    };
    pub const CCK: Country = Country {
        name: "Cocos (Keeling) Islands",
        iso2: Iso2::CC,
        iso3: Iso3::CCK,
    };
    pub const CHE: Country = Country {
        name: "Switzerland",
        iso2: Iso2::CH,
        iso3: Iso3::CHE,
    };
    pub const CHL: Country = Country {
        name: "Chile",
        iso2: Iso2::CL,
        iso3: Iso3::CHL,
    };
    pub const CHN: Country = Country {
        name: "China",
        iso2: Iso2::CN,
        iso3: Iso3::CHN,
    };
    pub const CIV: Country = Country {
        name: "Ivory Coast",
        iso2: Iso2::CI,
        iso3: Iso3::CIV,
    };
    pub const CMR: Country = Country {
        name: "Cameroon",
        iso2: Iso2::CM,
        iso3: Iso3::CMR,
    };
    pub const COD: Country = Country {
        name: "Democratic Republic of the Congo",
        iso2: Iso2::CD,
        iso3: Iso3::COD,
    };
    pub const COG: Country = Country {
        name: "Congo",
        iso2: Iso2::CG,
        iso3: Iso3::COG,
    };
    pub const COK: Country = Country {
        name: "Cook Islands",
        iso2: Iso2::CK,
        iso3: Iso3::COK,
    };
    pub const COL: Country = Country {
        name: "Colombia",
        iso2: Iso2::CO,
        iso3: Iso3::COL,
    };
    pub const COM: Country = Country {
        name: "Comoros",
        iso2: Iso2::KM,
        iso3: Iso3::COM,
    };
    pub const CPV: Country = Country {
        name: "Cape Verde",
        iso2: Iso2::CV,
        iso3: Iso3::CPV,
    };
    pub const CRI: Country = Country {
        name: "Costa Rica",
        iso2: Iso2::CR,
        iso3: Iso3::CRI,
    };
    pub const CUB: Country = Country {
        name: "Cuba",
        iso2: Iso2::CU,
        iso3: Iso3::CUB,
    };
    pub const CXR: Country = Country {
        name: "Christmas Islands",
        iso2: Iso2::CX,
        iso3: Iso3::CXR,
    };
    pub const CYM: Country = Country {
        name: "Cayman Islands",
        iso2: Iso2::KY,
        iso3: Iso3::CYM,
    };
    pub const CYP: Country = Country {
        name: "Cyprus",
        iso2: Iso2::CY,
        iso3: Iso3::CYP,
    };
    pub const CZE: Country = Country {
        name: "Czech Republic",
        iso2: Iso2::CZ,
        iso3: Iso3::CZE,
    };

    pub const DEU: Country = Country {
        name: "Germany",
        iso2: Iso2::DE,
        iso3: Iso3::DEU,
    };
    pub const DJI: Country = Country {
        name: "Djibouti",
        iso2: Iso2::DJ,
        iso3: Iso3::DJI,
    };
    pub const DMA: Country = Country {
        name: "Dominica",
        iso2: Iso2::DM,
        iso3: Iso3::DMA,
    };
    pub const DNK: Country = Country {
        name: "Denmark",
        iso2: Iso2::DK,
        iso3: Iso3::DNK,
    };
    pub const DOM: Country = Country {
        name: "Dominican Republic",
        iso2: Iso2::DO,
        iso3: Iso3::DOM,
    };
    pub const DZA: Country = Country {
        name: "Algeria",
        iso2: Iso2::DZ,
        iso3: Iso3::DZA,
    };

    pub const ECU: Country = Country {
        name: "Ecuador",
        iso2: Iso2::EC,
        iso3: Iso3::ECU,
    };
    pub const EGY: Country = Country {
        name: "Egypt",
        iso2: Iso2::EG,
        iso3: Iso3::EGY,
    };
    pub const ERI: Country = Country {
        name: "Eritrea",
        iso2: Iso2::ER,
        iso3: Iso3::ERI,
    };
    pub const ESH: Country = Country {
        name: "Western Sahara",
        iso2: Iso2::EH,
        iso3: Iso3::ESH,
    };
    pub const ESP: Country = Country {
        name: "Spain",
        iso2: Iso2::ES,
        iso3: Iso3::ESP,
    };
    pub const EST: Country = Country {
        name: "Estonia",
        iso2: Iso2::EE,
        iso3: Iso3::EST,
    };
    pub const ETH: Country = Country {
        name: "Ethiopia",
        iso2: Iso2::ET,
        iso3: Iso3::ETH,
    };

    pub const FIN: Country = Country {
        name: "Finland",
        iso2: Iso2::FI,
        iso3: Iso3::FIN,
    };
    pub const FJI: Country = Country {
        name: "Fiji",
        iso2: Iso2::FJ,
        iso3: Iso3::FJI,
    };
    pub const FLK: Country = Country {
        name: "Falkland Islands",
        iso2: Iso2::FK,
        iso3: Iso3::FLK,
    };
    pub const FRA: Country = Country {
        name: "France",
        iso2: Iso2::FR,
        iso3: Iso3::FRA,
    };
    pub const FRO: Country = Country {
        name: "Faroe Islands",
        iso2: Iso2::FO,
        iso3: Iso3::FRO,
    };
    pub const FM: Country = Country {
        name: "Federated States of Micronesia",
        iso2: Iso2::FM,
        iso3: Iso3::FSM,
    };

    pub const GAB: Country = Country {
        name: "Gabon",
        iso2: Iso2::GA,
        iso3: Iso3::GAB,
    };
    pub const GB: Country = Country {
        name: "United Kingdom",
        iso2: Iso2::GB,
        iso3: Iso3::GBR,
    };
    pub const GEO: Country = Country {
        name: "Georgia",
        iso2: Iso2::GE,
        iso3: Iso3::GEO,
    };
    pub const GGY: Country = Country {
        name: "Guernsey",
        iso2: Iso2::GG,
        iso3: Iso3::GGY,
    };
    pub const GHA: Country = Country {
        name: "Ghana",
        iso2: Iso2::GH,
        iso3: Iso3::GHA,
    };
    pub const GIB: Country = Country {
        name: "Gibraltar",
        iso2: Iso2::GI,
        iso3: Iso3::GIB,
    };
    pub const GIN: Country = Country {
        name: "Guinea",
        iso2: Iso2::GN,
        iso3: Iso3::GIN,
    };
    pub const GLP: Country = Country {
        name: "Guadeloupe",
        iso2: Iso2::GP,
        iso3: Iso3::GLP,
    };
    pub const GMB: Country = Country {
        name: "Gambia",
        iso2: Iso2::GM,
        iso3: Iso3::GMB,
    };
    pub const GNB: Country = Country {
        name: "Guinea - Bissau",
        iso2: Iso2::GW,
        iso3: Iso3::GNB,
    };
    pub const GNQ: Country = Country {
        name: "Equatorial Guinea",
        iso2: Iso2::GQ,
        iso3: Iso3::GNQ,
    };
    pub const GRC: Country = Country {
        name: "Greece",
        iso2: Iso2::GR,
        iso3: Iso3::GRC,
    };
    pub const GRD: Country = Country {
        name: "Grenada",
        iso2: Iso2::GD,
        iso3: Iso3::GRD,
    };
    pub const GRL: Country = Country {
        name: "Greenland",
        iso2: Iso2::GL,
        iso3: Iso3::GRL,
    };
    pub const GTM: Country = Country {
        name: "Guatemala",
        iso2: Iso2::GT,
        iso3: Iso3::GTM,
    };
    pub const CUF: Country = Country {
        name: "French Guiana",
        iso2: Iso2::GF,
        iso3: Iso3::GUF,
    };
    pub const GUM: Country = Country {
        name: "Guam",
        iso2: Iso2::GU,
        iso3: Iso3::GUM,
    };
    pub const GUY: Country = Country {
        name: "Guyana",
        iso2: Iso2::GY,
        iso3: Iso3::GUY,
    };

    pub const HKG: Country = Country {
        name: "Hong Kong",
        iso2: Iso2::HK,
        iso3: Iso3::HKG,
    };
    pub const HMD: Country = Country {
        name: "Heard Island and McDonald Islands",
        iso2: Iso2::HM,
        iso3: Iso3::HMD,
    };
    pub const HND: Country = Country {
        name: "Honduras",
        iso2: Iso2::HN,
        iso3: Iso3::HND,
    };
    pub const HRV: Country = Country {
        name: "Croatia",
        iso2: Iso2::HR,
        iso3: Iso3::HRV,
    };
    pub const HTI: Country = Country {
        name: "Haiti",
        iso2: Iso2::HT,
        iso3: Iso3::HTI,
    };
    pub const HUN: Country = Country {
        name: "Hungary",
        iso2: Iso2::HU,
        iso3: Iso3::HUN,
    };

    pub const IDN: Country = Country {
        name: "Indonesia",
        iso2: Iso2::ID,
        iso3: Iso3::IDN,
    };
    pub const IMN: Country = Country {
        name: "Isle of Man",
        iso2: Iso2::IM,
        iso3: Iso3::IMN,
    };
    pub const IND: Country = Country {
        name: "India",
        iso2: Iso2::IN,
        iso3: Iso3::IND,
    };
    pub const IOT: Country = Country {
        name: "British Indian Ocean Territory",
        iso2: Iso2::IO,
        iso3: Iso3::IOT,
    };
    pub const IRL: Country = Country {
        name: "Ireland",
        iso2: Iso2::IE,
        iso3: Iso3::IRL,
    };
    pub const IRN: Country = Country {
        name: "Iran",
        iso2: Iso2::IR,
        iso3: Iso3::IRN,
    };
    pub const IRQ: Country = Country {
        name: "Iraq",
        iso2: Iso2::IQ,
        iso3: Iso3::IRQ,
    };
    pub const ISL: Country = Country {
        name: "Iceland",
        iso2: Iso2::IS,
        iso3: Iso3::ISL,
    };
    pub const ISR: Country = Country {
        name: "Israel",
        iso2: Iso2::IL,
        iso3: Iso3::ISR,
    };
    pub const ITA: Country = Country {
        name: "Italy",
        iso2: Iso2::IT,
        iso3: Iso3::ITA,
    };

    pub const JAM: Country = Country {
        name: "Jamaica",
        iso2: Iso2::JM,
        iso3: Iso3::JAM,
    };
    pub const JEY: Country = Country {
        name: "Jersey",
        iso2: Iso2::JE,
        iso3: Iso3::JEY,
    };
    pub const JOR: Country = Country {
        name: "Jordan",
        iso2: Iso2::JO,
        iso3: Iso3::JOR,
    };
    pub const JPN: Country = Country {
        name: "Japan",
        iso2: Iso2::JP,
        iso3: Iso3::JPN,
    };

    pub const KAZ: Country = Country {
        name: "Kazakhstan",
        iso2: Iso2::KZ,
        iso3: Iso3::KAZ,
    };
    pub const KEN: Country = Country {
        name: "Kenya",
        iso2: Iso2::KE,
        iso3: Iso3::KEN,
    };
    pub const KGZ: Country = Country {
        name: "Kyrgyzstan",
        iso2: Iso2::KG,
        iso3: Iso3::KGZ,
    };
    pub const KHM: Country = Country {
        name: "Cambodia",
        iso2: Iso2::KH,
        iso3: Iso3::KHM,
    };
    pub const KIR: Country = Country {
        name: "Kiribati",
        iso2: Iso2::KI,
        iso3: Iso3::KIR,
    };
    pub const KNA: Country = Country {
        name: "Saint Kitts and Nevis",
        iso2: Iso2::KN,
        iso3: Iso3::KNA,
    };
    pub const KOR: Country = Country {
        name: "South Korea",
        iso2: Iso2::KR,
        iso3: Iso3::KOR,
    };
    pub const KWT: Country = Country {
        name: "Kuwait",
        iso2: Iso2::KW,
        iso3: Iso3::KWT,
    };

    pub const LAO: Country = Country {
        name: "Laos",
        iso2: Iso2::LA,
        iso3: Iso3::LAO,
    };
    pub const LBN: Country = Country {
        name: "Lebanon",
        iso2: Iso2::LB,
        iso3: Iso3::LBN,
    };
    pub const LBR: Country = Country {
        name: "Liberia",
        iso2: Iso2::LR,
        iso3: Iso3::LBR,
    };
    pub const LBY: Country = Country {
        name: "Libya",
        iso2: Iso2::LY,
        iso3: Iso3::LBY,
    };
    pub const LCA: Country = Country {
        name: "Saint Lucia",
        iso2: Iso2::LC,
        iso3: Iso3::LCA,
    };
    pub const LIE: Country = Country {
        name: "Liechtenstein",
        iso2: Iso2::LI,
        iso3: Iso3::LIE,
    };
    pub const LKA: Country = Country {
        name: "Sri Lanka",
        iso2: Iso2::LK,
        iso3: Iso3::LKA,
    };
    pub const LSO: Country = Country {
        name: "Lesotho",
        iso2: Iso2::LS,
        iso3: Iso3::LSO,
    };
    pub const LTU: Country = Country {
        name: "Lithuania",
        iso2: Iso2::LT,
        iso3: Iso3::LTU,
    };
    pub const LUX: Country = Country {
        name: "Luxembourg",
        iso2: Iso2::LU,
        iso3: Iso3::LUX,
    };
    pub const LVA: Country = Country {
        name: "LVA",
        iso2: Iso2::LV,
        iso3: Iso3::LVA,
    };

    pub const MAC: Country = Country {
        name: "Macao",
        iso2: Iso2::MO,
        iso3: Iso3::MAC,
    };
    pub const MAF: Country = Country {
        name: "Saint Martin",
        iso2: Iso2::MF,
        iso3: Iso3::MAF,
    };
    pub const MAR: Country = Country {
        name: "Morocco",
        iso2: Iso2::MA,
        iso3: Iso3::MAR,
    };
    pub const MCO: Country = Country {
        name: "Monaco",
        iso2: Iso2::MC,
        iso3: Iso3::MCO,
    };
    pub const MDA: Country = Country {
        name: "Moldova",
        iso2: Iso2::MD,
        iso3: Iso3::MDA,
    };
    pub const MDG: Country = Country {
        name: "Madagascar",
        iso2: Iso2::MG,
        iso3: Iso3::MDG,
    };
    pub const MDV: Country = Country {
        name: "Maldives",
        iso2: Iso2::MV,
        iso3: Iso3::MDV,
    };
    pub const MEX: Country = Country {
        name: "Mexico",
        iso2: Iso2::MX,
        iso3: Iso3::MEX,
    };
    pub const MHL: Country = Country {
        name: "Marshall Islands",
        iso2: Iso2::MH,
        iso3: Iso3::MHL,
    };
    pub const MKD: Country = Country {
        name: "North Macedonia",
        iso2: Iso2::MK,
        iso3: Iso3::MKD,
    };
    pub const MLI: Country = Country {
        name: "Mali",
        iso2: Iso2::ML,
        iso3: Iso3::MLI,
    };
    pub const MLT: Country = Country {
        name: "Malta",
        iso2: Iso2::MT,
        iso3: Iso3::MLT,
    };
    pub const MMR: Country = Country {
        name: "Myanmar",
        iso2: Iso2::MM,
        iso3: Iso3::MMR,
    };
    pub const MNE: Country = Country {
        name: "Montenegro",
        iso2: Iso2::ME,
        iso3: Iso3::MNE,
    };
    pub const MNG: Country = Country {
        name: "Mongolia",
        iso2: Iso2::MN,
        iso3: Iso3::MNG,
    };
    pub const MNP: Country = Country {
        name: "Northern Mariana Islands",
        iso2: Iso2::MP,
        iso3: Iso3::MNP,
    };
    pub const MOZ: Country = Country {
        name: "Mozambique",
        iso2: Iso2::MZ,
        iso3: Iso3::MOZ,
    };
    pub const MRT: Country = Country {
        name: "Mauritania",
        iso2: Iso2::MR,
        iso3: Iso3::MRT,
    };
    pub const MS: Country = Country {
        name: "Montserrat",
        iso2: Iso2::MS,
        iso3: Iso3::MSR,
    };
    pub const MTQ: Country = Country {
        name: "Martinique",
        iso2: Iso2::MQ,
        iso3: Iso3::MTQ,
    };
    pub const MUS: Country = Country {
        name: "Mauritius",
        iso2: Iso2::MU,
        iso3: Iso3::MUS,
    };
    pub const MWI: Country = Country {
        name: "Malawi",
        iso2: Iso2::MI,
        iso3: Iso3::MWI,
    };
    pub const MYS: Country = Country {
        name: "Malaysia",
        iso2: Iso2::MY,
        iso3: Iso3::MYS,
    };
    pub const MYT: Country = Country {
        name: "Mayotte",
        iso2: Iso2::YT,
        iso3: Iso3::MYT,
    };

    pub const NAM: Country = Country {
        name: "Namibia",
        iso2: Iso2::NA,
        iso3: Iso3::NAM,
    };
    pub const NCL: Country = Country {
        name: "New Caledonia",
        iso2: Iso2::NC,
        iso3: Iso3::NCL,
    };
    pub const NER: Country = Country {
        name: "Niger",
        iso2: Iso2::NE,
        iso3: Iso3::NER,
    };
    pub const NFK: Country = Country {
        name: "Norfolk Island",
        iso2: Iso2::NF,
        iso3: Iso3::NFK,
    };
    pub const NGA: Country = Country {
        name: "Nigeria",
        iso2: Iso2::NG,
        iso3: Iso3::NGA,
    };
    pub const NIC: Country = Country {
        name: "Nicaragua",
        iso2: Iso2::NI,
        iso3: Iso3::NIC,
    };
    pub const NIU: Country = Country {
        name: "Niue",
        iso2: Iso2::NU,
        iso3: Iso3::NIU,
    };
    pub const NLD: Country = Country {
        name: "Netherlands",
        iso2: Iso2::NL,
        iso3: Iso3::NLD,
    };
    pub const NOR: Country = Country {
        name: "Norway",
        iso2: Iso2::NO,
        iso3: Iso3::NOR,
    };
    pub const NPL: Country = Country {
        name: "Nepal",
        iso2: Iso2::NP,
        iso3: Iso3::NPL,
    };
    pub const NRU: Country = Country {
        name: "Nauru",
        iso2: Iso2::NR,
        iso3: Iso3::NRU,
    };
    pub const NZL: Country = Country {
        name: "New Zealand",
        iso2: Iso2::NZ,
        iso3: Iso3::NZL,
    };

    pub const OMN: Country = Country {
        name: "Oman",
        iso2: Iso2::OM,
        iso3: Iso3::OMN,
    };

    pub const PAK: Country = Country {
        name: "Pakistan",
        iso2: Iso2::PK,
        iso3: Iso3::PAK,
    };
    pub const PAN: Country = Country {
        name: "Panama",
        iso2: Iso2::PA,
        iso3: Iso3::PAN,
    };
    pub const PCN: Country = Country {
        name: "Pitcairn",
        iso2: Iso2::PN,
        iso3: Iso3::PCN,
    };
    pub const PER: Country = Country {
        name: "Peru",
        iso2: Iso2::PE,
        iso3: Iso3::PER,
    };
    pub const PHL: Country = Country {
        name: "Philippines",
        iso2: Iso2::PH,
        iso3: Iso3::PHL,
    };
    pub const PLW: Country = Country {
        name: "Palau",
        iso2: Iso2::PW,
        iso3: Iso3::PLW,
    };
    pub const PNG: Country = Country {
        name: "Papua New Guinea",
        iso2: Iso2::PG,
        iso3: Iso3::PNG,
    };
    pub const POL: Country = Country {
        name: "Poland",
        iso2: Iso2::PL,
        iso3: Iso3::POL,
    };
    pub const PRI: Country = Country {
        name: "Puerto Rico",
        iso2: Iso2::PR,
        iso3: Iso3::PRI,
    };
    pub const PRK: Country = Country {
        name: "North Korea",
        iso2: Iso2::KP,
        iso3: Iso3::PRK,
    };
    pub const PRT: Country = Country {
        name: "Portugal",
        iso2: Iso2::PT,
        iso3: Iso3::PRT,
    };
    pub const PRY: Country = Country {
        name: "Paraguay",
        iso2: Iso2::PY,
        iso3: Iso3::PRY,
    };
    pub const PSE: Country = Country {
        name: "Palestine",
        iso2: Iso2::PS,
        iso3: Iso3::PSE,
    };
    pub const PYF: Country = Country {
        name: "French Polynesia",
        iso2: Iso2::PF,
        iso3: Iso3::PYF,
    };

    pub const QAT: Country = Country {
        name: "Qatar",
        iso2: Iso2::QA,
        iso3: Iso3::QAT,
    };

    pub const REU: Country = Country {
        name: "Réunion Island",
        iso2: Iso2::RE,
        iso3: Iso3::REU,
    };
    pub const ROU: Country = Country {
        name: "Romania",
        iso2: Iso2::RO,
        iso3: Iso3::ROU,
    };
    pub const RUS: Country = Country {
        name: "Russia",
        iso2: Iso2::RU,
        iso3: Iso3::RUS,
    };
    pub const RWA: Country = Country {
        name: "Rwanda",
        iso2: Iso2::RW,
        iso3: Iso3::RWA,
    };

    pub const SAU: Country = Country {
        name: "Saudi Arabia",
        iso2: Iso2::SA,
        iso3: Iso3::SAU,
    };
    pub const SDN: Country = Country {
        name: "Sudan",
        iso2: Iso2::SD,
        iso3: Iso3::SDN,
    };
    pub const SEN: Country = Country {
        name: "Senegal",
        iso2: Iso2::SN,
        iso3: Iso3::SEN,
    };
    pub const SGP: Country = Country {
        name: "Singapore",
        iso2: Iso2::SG,
        iso3: Iso3::SGP,
    };
    pub const SGS: Country = Country {
        name: "South Georgia and the South Sandwich Islands",
        iso2: Iso2::GS,
        iso3: Iso3::SGS,
    };
    pub const SHN: Country = Country {
        name: "Saint Helena, Ascension and Tristan da Cunha",
        iso2: Iso2::SH,
        iso3: Iso3::SHN,
    };
    pub const SJM: Country = Country {
        name: "Svalbard and Jan Mayen",
        iso2: Iso2::SJ,
        iso3: Iso3::SJM,
    };
    pub const SLB: Country = Country {
        name: "Solomon Islands",
        iso2: Iso2::SB,
        iso3: Iso3::SLB,
    };
    pub const SLE: Country = Country {
        name: "Sierra Leone",
        iso2: Iso2::SL,
        iso3: Iso3::SLE,
    };
    pub const SLV: Country = Country {
        name: "El Salvador",
        iso2: Iso2::SV,
        iso3: Iso3::SLV,
    };
    pub const SMR: Country = Country {
        name: "San Marino",
        iso2: Iso2::SM,
        iso3: Iso3::SMR,
    };
    pub const SOM: Country = Country {
        name: "Somalia",
        iso2: Iso2::SO,
        iso3: Iso3::SOM,
    };
    pub const SPM: Country = Country {
        name: "Saint Pierre and Miquelon",
        iso2: Iso2::PM,
        iso3: Iso3::SPM,
    };
    pub const SRB: Country = Country {
        name: "Serbia",
        iso2: Iso2::RS,
        iso3: Iso3::SRB,
    };
    pub const STP: Country = Country {
        name: "Sao Tome and Principe",
        iso2: Iso2::ST,
        iso3: Iso3::STP,
    };
    pub const SUR: Country = Country {
        name: "Suriname",
        iso2: Iso2::SR,
        iso3: Iso3::SUR,
    };
    pub const SVK: Country = Country {
        name: "Slovakia",
        iso2: Iso2::SK,
        iso3: Iso3::SVK,
    };
    pub const SVN: Country = Country {
        name: "Slovenia",
        iso2: Iso2::SI,
        iso3: Iso3::SVN,
    };
    pub const SWE: Country = Country {
        name: "Sweden",
        iso2: Iso2::SE,
        iso3: Iso3::SWE,
    };
    pub const SWZ: Country = Country {
        name: "Swaziland",
        iso2: Iso2::SZ,
        iso3: Iso3::SWZ,
    };
    pub const SYC: Country = Country {
        name: "Seychelles",
        iso2: Iso2::SC,
        iso3: Iso3::SYC,
    };
    pub const SYR: Country = Country {
        name: "Syria",
        iso2: Iso2::SY,
        iso3: Iso3::SYR,
    };

    pub const TCA: Country = Country {
        name: "Turks and Caicos Islands",
        iso2: Iso2::TC,
        iso3: Iso3::TCA,
    };
    pub const TCD: Country = Country {
        name: "Chad",
        iso2: Iso2::TD,
        iso3: Iso3::TCD,
    };
    pub const TGO: Country = Country {
        name: "Togo",
        iso2: Iso2::TG,
        iso3: Iso3::TGO,
    };
    pub const THA: Country = Country {
        name: "Thailand",
        iso2: Iso2::TH,
        iso3: Iso3::THA,
    };
    pub const TJK: Country = Country {
        name: "Tajikistan",
        iso2: Iso2::TJ,
        iso3: Iso3::TJK,
    };
    pub const TKL: Country = Country {
        name: "Tokelau",
        iso2: Iso2::TK,
        iso3: Iso3::TKL,
    };
    pub const TKM: Country = Country {
        name: "Turkmenistan",
        iso2: Iso2::TM,
        iso3: Iso3::TKM,
    };
    pub const TLS: Country = Country {
        name: "Timor - Leste",
        iso2: Iso2::TL,
        iso3: Iso3::TLS,
    };
    pub const TON: Country = Country {
        name: "Tonga",
        iso2: Iso2::TO,
        iso3: Iso3::TON,
    };
    pub const TTO: Country = Country {
        name: "Trinidad and Tobago",
        iso2: Iso2::TT,
        iso3: Iso3::TTO,
    };
    pub const TUN: Country = Country {
        name: "Tunisia",
        iso2: Iso2::TN,
        iso3: Iso3::TUN,
    };
    pub const TUR: Country = Country {
        name: "Turkey",
        iso2: Iso2::TR,
        iso3: Iso3::TUR,
    };
    pub const TUV: Country = Country {
        name: "Tuvalu",
        iso2: Iso2::TV,
        iso3: Iso3::TUV,
    };
    pub const TWN: Country = Country {
        name: "Taiwan",
        iso2: Iso2::TW,
        iso3: Iso3::TWN,
    };
    pub const TZA: Country = Country {
        name: "Tanzania",
        iso2: Iso2::TZ,
        iso3: Iso3::TZA,
    };

    pub const UGA: Country = Country {
        name: "Uganda",
        iso2: Iso2::UG,
        iso3: Iso3::UGA,
    };
    pub const UKR: Country = Country {
        name: "Ukraine",
        iso2: Iso2::UA,
        iso3: Iso3::UKR,
    };
    pub const UMI: Country = Country {
        name: "United States Minor Outlying Islands",
        iso2: Iso2::UM,
        iso3: Iso3::UMI,
    };
    pub const URY: Country = Country {
        name: "Uruguay",
        iso2: Iso2::UY,
        iso3: Iso3::URY,
    };
    pub const USA: Country = Country {
        name: "United States",
        iso2: Iso2::US,
        iso3: Iso3::USA,
    };
    pub const UZB: Country = Country {
        name: "Uzbekistan",
        iso2: Iso2::UZ,
        iso3: Iso3::UZB,
    };

    pub const VAT: Country = Country {
        name: "Holy See",
        iso2: Iso2::VA,
        iso3: Iso3::VAT,
    };
    pub const VCT: Country = Country {
        name: "Saint Vincent and the Grenadines",
        iso2: Iso2::VC,
        iso3: Iso3::VCT,
    };
    pub const VEN: Country = Country {
        name: "Venezuela",
        iso2: Iso2::VE,
        iso3: Iso3::VEN,
    };
    pub const VGB: Country = Country {
        name: "British Virgin Islands",
        iso2: Iso2::VG,
        iso3: Iso3::VGB,
    };
    pub const VIR: Country = Country {
        name: "U.S. Virgin Islands",
        iso2: Iso2::VI,
        iso3: Iso3::VIR,
    };
    pub const VNM: Country = Country {
        name: "Vietnam",
        iso2: Iso2::VN,
        iso3: Iso3::VNM,
    };
    pub const VUT: Country = Country {
        name: "Vanuatu",
        iso2: Iso2::VU,
        iso3: Iso3::VUT,
    };

    pub const WLF: Country = Country {
        name: "Wallis and Futuna",
        iso2: Iso2::WF,
        iso3: Iso3::WLF,
    };
    pub const WSM: Country = Country {
        name: "Samoa",
        iso2: Iso2::WS,
        iso3: Iso3::WSM,
    };

    pub const YEM: Country = Country {
        name: "Yemen",
        iso2: Iso2::YE,
        iso3: Iso3::YEM,
    };

    pub const ZAF: Country = Country {
        name: "South Africa",
        iso2: Iso2::ZA,
        iso3: Iso3::ZAF,
    };
    pub const ZMB: Country = Country {
        name: "Zambia",
        iso2: Iso2::ZM,
        iso3: Iso3::ZMB,
    };
    pub const ZWE: Country = Country {
        name: "Zimbabwe",
        iso2: Iso2::ZW,
        iso3: Iso3::ZWE,
    };
}
