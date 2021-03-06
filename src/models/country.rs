use std::slice::Iter;

use crate::models::iso::{Iso2, Iso3};
use crate::models::top_level_domain::TopLevelDomain;
use crate::regions::ALL_COUNTRIES;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Country {
    pub name: &'static str,
    pub iso2: Iso2,
    pub iso3: Iso3,
    pub top_level_domain: TopLevelDomain,
    // country_code (str)
    // calling code (nr)
    // date format (str)
    // capital (str)
}

pub const NUMBER_OF_COUNTRIES: u8 = 251;

impl Country {
    pub const ABW: Country = Country {
        name: "Aruba",
        iso2: Iso2::AW,
        iso3: Iso3::ABW,
        top_level_domain: TopLevelDomain::AW,
    };
    pub const AFG: Country = Country {
        name: "Afghanistan",
        iso2: Iso2::AF,
        iso3: Iso3::AFG,
        top_level_domain: TopLevelDomain::AF,
    };
    pub const AGO: Country = Country {
        name: "Angola",
        iso2: Iso2::AO,
        iso3: Iso3::AGO,
        top_level_domain: TopLevelDomain::AO,
    };
    pub const AIA: Country = Country {
        name: "Anguilla",
        iso2: Iso2::AI,
        iso3: Iso3::AIA,
        top_level_domain: TopLevelDomain::AI,
    };
    pub const ALA: Country = Country {
        name: "Åland Islands",
        iso2: Iso2::AX,
        iso3: Iso3::ALA,
        top_level_domain: TopLevelDomain::AX,
    };
    pub const ALB: Country = Country {
        name: "Albania",
        iso2: Iso2::AL,
        iso3: Iso3::ALB,
        top_level_domain: TopLevelDomain::AL,
    };
    pub const AND: Country = Country {
        name: "Andorra",
        iso2: Iso2::AD,
        iso3: Iso3::AND,
        top_level_domain: TopLevelDomain::AD,
    };
    pub const ANT: Country = Country {
        name: "Netherlands Antilles",
        iso2: Iso2::AN,
        iso3: Iso3::ANT,
        top_level_domain: TopLevelDomain::AN,
    };
    pub const ARE: Country = Country {
        name: "United Arab Emirates",
        iso2: Iso2::AE,
        iso3: Iso3::ARE,
        top_level_domain: TopLevelDomain::AE,
    };
    pub const ARG: Country = Country {
        name: "Argentina",
        iso2: Iso2::AR,
        iso3: Iso3::ARG,
        top_level_domain: TopLevelDomain::AR,
    };
    pub const ARM: Country = Country {
        name: "Armenia",
        iso2: Iso2::AM,
        iso3: Iso3::ARM,
        top_level_domain: TopLevelDomain::AM,
    };
    pub const ASM: Country = Country {
        name: "American Samoa",
        iso2: Iso2::AS,
        iso3: Iso3::ASM,
        top_level_domain: TopLevelDomain::AS,
    };
    pub const ATA: Country = Country {
        name: "Antarctica",
        iso2: Iso2::AQ,
        iso3: Iso3::ATA,
        top_level_domain: TopLevelDomain::AQ,
    };
    pub const ATF: Country = Country {
        name: "French Southern Territories",
        iso2: Iso2::TF,
        iso3: Iso3::ATF,
        top_level_domain: TopLevelDomain::TF,
    };
    pub const ATG: Country = Country {
        name: "Antigua and Barbuda",
        iso2: Iso2::AG,
        iso3: Iso3::ATG,
        top_level_domain: TopLevelDomain::AG,
    };
    pub const AUS: Country = Country {
        name: "Australia",
        iso2: Iso2::AU,
        iso3: Iso3::AUS,
        top_level_domain: TopLevelDomain::AU,
    };
    pub const AUT: Country = Country {
        name: "Austria",
        iso2: Iso2::AT,
        iso3: Iso3::AUT,
        top_level_domain: TopLevelDomain::AT,
    };
    pub const AZE: Country = Country {
        name: "Azerbaijan",
        iso2: Iso2::AZ,
        iso3: Iso3::AZE,
        top_level_domain: TopLevelDomain::AZ,
    };

    pub const BDI: Country = Country {
        name: "Burundi",
        iso2: Iso2::BI,
        iso3: Iso3::BDI,
        top_level_domain: TopLevelDomain::BI,
    };
    pub const BEL: Country = Country {
        name: "Belgium",
        iso2: Iso2::BE,
        iso3: Iso3::BEL,
        top_level_domain: TopLevelDomain::BE,
    };
    pub const BEN: Country = Country {
        name: "Benin",
        iso2: Iso2::BJ,
        iso3: Iso3::BEN,
        top_level_domain: TopLevelDomain::BJ,
    };
    pub const BES: Country = Country {
        name: "Caribbean Netherlands",
        iso2: Iso2::BQ,
        iso3: Iso3::BES,
        top_level_domain: TopLevelDomain::BQ,
    };

    pub const BFA: Country = Country {
        name: "Burkina Faso",
        iso2: Iso2::BF,
        iso3: Iso3::BFA,
        top_level_domain: TopLevelDomain::BF,
    };
    pub const BGD: Country = Country {
        name: "Bangladesh",
        iso2: Iso2::BD,
        iso3: Iso3::BGD,
        top_level_domain: TopLevelDomain::BD,
    };
    pub const BGR: Country = Country {
        name: "Bulgaria",
        iso2: Iso2::BG,
        iso3: Iso3::BGR,
        top_level_domain: TopLevelDomain::BG,
    };
    pub const BHR: Country = Country {
        name: "Bahrain",
        iso2: Iso2::BH,
        iso3: Iso3::BHR,
        top_level_domain: TopLevelDomain::BH,
    };
    pub const BHS: Country = Country {
        name: "Bahamas",
        iso2: Iso2::BS,
        iso3: Iso3::BHS,
        top_level_domain: TopLevelDomain::BS,
    };
    pub const BIH: Country = Country {
        name: "Bosnia and Herzegovina",
        iso2: Iso2::BA,
        iso3: Iso3::BIH,
        top_level_domain: TopLevelDomain::BA,
    };
    pub const BLM: Country = Country {
        name: "Saint Barthélemy",
        iso2: Iso2::BL,
        iso3: Iso3::BLM,
        top_level_domain: TopLevelDomain::BL,
    };
    pub const BLR: Country = Country {
        name: "Belarus",
        iso2: Iso2::BY,
        iso3: Iso3::BLR,
        top_level_domain: TopLevelDomain::BY,
    };
    pub const BLZ: Country = Country {
        name: "Belize",
        iso2: Iso2::BZ,
        iso3: Iso3::BLZ,
        top_level_domain: TopLevelDomain::BZ,
    };
    pub const BMU: Country = Country {
        name: "Bermuda",
        iso2: Iso2::BM,
        iso3: Iso3::BMU,
        top_level_domain: TopLevelDomain::BM,
    };
    pub const BOL: Country = Country {
        name: "Bolivia",
        iso2: Iso2::BO,
        iso3: Iso3::BOL,
        top_level_domain: TopLevelDomain::BO,
    };
    pub const BRA: Country = Country {
        name: "Brazil",
        iso2: Iso2::BR,
        iso3: Iso3::BRA,
        top_level_domain: TopLevelDomain::BR,
    };
    pub const BRB: Country = Country {
        name: "Barbados",
        iso2: Iso2::BB,
        iso3: Iso3::BRB,
        top_level_domain: TopLevelDomain::BB,
    };
    pub const BRN: Country = Country {
        name: "Brunei Darussalam",
        iso2: Iso2::BN,
        iso3: Iso3::BRN,
        top_level_domain: TopLevelDomain::BN,
    };
    pub const BTN: Country = Country {
        name: "Bhutan",
        iso2: Iso2::BT,
        iso3: Iso3::BTN,
        top_level_domain: TopLevelDomain::BT,
    };
    pub const BVT: Country = Country {
        name: "Bouvet Island",
        iso2: Iso2::BV,
        iso3: Iso3::BVT,
        top_level_domain: TopLevelDomain::BY,
    };
    pub const BWA: Country = Country {
        name: "Botswana",
        iso2: Iso2::BW,
        iso3: Iso3::BWA,
        top_level_domain: TopLevelDomain::BW,
    };

    pub const CAF: Country = Country {
        name: "Central African Republic",
        iso2: Iso2::CF,
        iso3: Iso3::CAF,
        top_level_domain: TopLevelDomain::CF,
    };
    pub const CAN: Country = Country {
        name: "Canada",
        iso2: Iso2::CA,
        iso3: Iso3::CAN,
        top_level_domain: TopLevelDomain::CA,
    };
    pub const CCK: Country = Country {
        name: "Cocos (Keeling) Islands",
        iso2: Iso2::CC,
        iso3: Iso3::CCK,
        top_level_domain: TopLevelDomain::CC,
    };
    pub const CHE: Country = Country {
        name: "Switzerland",
        iso2: Iso2::CH,
        iso3: Iso3::CHE,
        top_level_domain: TopLevelDomain::CH,
    };
    pub const CHL: Country = Country {
        name: "Chile",
        iso2: Iso2::CL,
        iso3: Iso3::CHL,
        top_level_domain: TopLevelDomain::CL,
    };
    pub const CHN: Country = Country {
        name: "China",
        iso2: Iso2::CN,
        iso3: Iso3::CHN,
        top_level_domain: TopLevelDomain::CN,
    };
    pub const CIV: Country = Country {
        name: "Ivory Coast",
        iso2: Iso2::CI,
        iso3: Iso3::CIV,
        top_level_domain: TopLevelDomain::CI,
    };
    pub const CMR: Country = Country {
        name: "Cameroon",
        iso2: Iso2::CM,
        iso3: Iso3::CMR,
        top_level_domain: TopLevelDomain::CM,
    };
    pub const COD: Country = Country {
        name: "Democratic Republic of the Congo",
        iso2: Iso2::CD,
        iso3: Iso3::COD,
        top_level_domain: TopLevelDomain::CD,
    };
    pub const COG: Country = Country {
        name: "Congo",
        iso2: Iso2::CG,
        iso3: Iso3::COG,
        top_level_domain: TopLevelDomain::CG,
    };
    pub const COK: Country = Country {
        name: "Cook Islands",
        iso2: Iso2::CK,
        iso3: Iso3::COK,
        top_level_domain: TopLevelDomain::CK,
    };
    pub const COL: Country = Country {
        name: "Colombia",
        iso2: Iso2::CO,
        iso3: Iso3::COL,
        top_level_domain: TopLevelDomain::CO,
    };
    pub const COM: Country = Country {
        name: "Comoros",
        iso2: Iso2::KM,
        iso3: Iso3::COM,
        top_level_domain: TopLevelDomain::KM,
    };
    pub const CPV: Country = Country {
        name: "Cape Verde",
        iso2: Iso2::CV,
        iso3: Iso3::CPV,
        top_level_domain: TopLevelDomain::CV,
    };
    pub const CRI: Country = Country {
        name: "Costa Rica",
        iso2: Iso2::CR,
        iso3: Iso3::CRI,
        top_level_domain: TopLevelDomain::CR,
    };
    pub const CUB: Country = Country {
        name: "Cuba",
        iso2: Iso2::CU,
        iso3: Iso3::CUB,
        top_level_domain: TopLevelDomain::CU,
    };
    pub const CUW: Country = Country {
        name: "Curaçao",
        iso2: Iso2::CW,
        iso3: Iso3::CUW,
        top_level_domain: TopLevelDomain::CW,
    };
    pub const CXR: Country = Country {
        name: "Christmas Islands",
        iso2: Iso2::CX,
        iso3: Iso3::CXR,
        top_level_domain: TopLevelDomain::CX,
    };
    pub const CYM: Country = Country {
        name: "Cayman Islands",
        iso2: Iso2::KY,
        iso3: Iso3::CYM,
        top_level_domain: TopLevelDomain::KY,
    };
    pub const CYP: Country = Country {
        name: "Cyprus",
        iso2: Iso2::CY,
        iso3: Iso3::CYP,
        top_level_domain: TopLevelDomain::CY,
    };
    pub const CZE: Country = Country {
        name: "Czech Republic",
        iso2: Iso2::CZ,
        iso3: Iso3::CZE,
        top_level_domain: TopLevelDomain::CZ,
    };

    pub const DEU: Country = Country {
        name: "Germany",
        iso2: Iso2::DE,
        iso3: Iso3::DEU,
        top_level_domain: TopLevelDomain::DE,
    };
    pub const DJI: Country = Country {
        name: "Djibouti",
        iso2: Iso2::DJ,
        iso3: Iso3::DJI,
        top_level_domain: TopLevelDomain::DJ,
    };
    pub const DMA: Country = Country {
        name: "Dominica",
        iso2: Iso2::DM,
        iso3: Iso3::DMA,
        top_level_domain: TopLevelDomain::DM,
    };
    pub const DNK: Country = Country {
        name: "Denmark",
        iso2: Iso2::DK,
        iso3: Iso3::DNK,
        top_level_domain: TopLevelDomain::DK,
    };
    pub const DOM: Country = Country {
        name: "Dominican Republic",
        iso2: Iso2::DO,
        iso3: Iso3::DOM,
        top_level_domain: TopLevelDomain::DO,
    };
    pub const DZA: Country = Country {
        name: "Algeria",
        iso2: Iso2::DZ,
        iso3: Iso3::DZA,
        top_level_domain: TopLevelDomain::DZ,
    };

    pub const ECU: Country = Country {
        name: "Ecuador",
        iso2: Iso2::EC,
        iso3: Iso3::ECU,
        top_level_domain: TopLevelDomain::EC,
    };
    pub const EGY: Country = Country {
        name: "Egypt",
        iso2: Iso2::EG,
        iso3: Iso3::EGY,
        top_level_domain: TopLevelDomain::EG,
    };
    pub const ERI: Country = Country {
        name: "Eritrea",
        iso2: Iso2::ER,
        iso3: Iso3::ERI,
        top_level_domain: TopLevelDomain::ER,
    };
    pub const ESH: Country = Country {
        name: "Western Sahara",
        iso2: Iso2::EH,
        iso3: Iso3::ESH,
        top_level_domain: TopLevelDomain::EH,
    };
    pub const ESP: Country = Country {
        name: "Spain",
        iso2: Iso2::ES,
        iso3: Iso3::ESP,
        top_level_domain: TopLevelDomain::ES,
    };
    pub const EST: Country = Country {
        name: "Estonia",
        iso2: Iso2::EE,
        iso3: Iso3::EST,
        top_level_domain: TopLevelDomain::EE,
    };
    pub const ETH: Country = Country {
        name: "Ethiopia",
        iso2: Iso2::ET,
        iso3: Iso3::ETH,
        top_level_domain: TopLevelDomain::ET,
    };

    pub const FIN: Country = Country {
        name: "Finland",
        iso2: Iso2::FI,
        iso3: Iso3::FIN,
        top_level_domain: TopLevelDomain::FI,
    };
    pub const FJI: Country = Country {
        name: "Fiji",
        iso2: Iso2::FJ,
        iso3: Iso3::FJI,
        top_level_domain: TopLevelDomain::FJ,
    };
    pub const FLK: Country = Country {
        name: "Falkland Islands",
        iso2: Iso2::FK,
        iso3: Iso3::FLK,
        top_level_domain: TopLevelDomain::FK,
    };
    pub const FRA: Country = Country {
        name: "France",
        iso2: Iso2::FR,
        iso3: Iso3::FRA,
        top_level_domain: TopLevelDomain::FR,
    };
    pub const FRO: Country = Country {
        name: "Faroe Islands",
        iso2: Iso2::FO,
        iso3: Iso3::FRO,
        top_level_domain: TopLevelDomain::FO,
    };
    pub const FSM: Country = Country {
        name: "Federated States of Micronesia",
        iso2: Iso2::FM,
        iso3: Iso3::FSM,
        top_level_domain: TopLevelDomain::FM,
    };

    pub const GAB: Country = Country {
        name: "Gabon",
        iso2: Iso2::GA,
        iso3: Iso3::GAB,
        top_level_domain: TopLevelDomain::GA,
    };
    pub const GBR: Country = Country {
        name: "United Kingdom",
        iso2: Iso2::GB,
        iso3: Iso3::GBR,
        top_level_domain: TopLevelDomain::GB,
    };
    pub const GEO: Country = Country {
        name: "Georgia",
        iso2: Iso2::GE,
        iso3: Iso3::GEO,
        top_level_domain: TopLevelDomain::GE,
    };
    pub const GGY: Country = Country {
        name: "Guernsey",
        iso2: Iso2::GG,
        iso3: Iso3::GGY,
        top_level_domain: TopLevelDomain::GG,
    };
    pub const GHA: Country = Country {
        name: "Ghana",
        iso2: Iso2::GH,
        iso3: Iso3::GHA,
        top_level_domain: TopLevelDomain::GH,
    };
    pub const GIB: Country = Country {
        name: "Gibraltar",
        iso2: Iso2::GI,
        iso3: Iso3::GIB,
        top_level_domain: TopLevelDomain::GI,
    };
    pub const GIN: Country = Country {
        name: "Guinea",
        iso2: Iso2::GN,
        iso3: Iso3::GIN,
        top_level_domain: TopLevelDomain::GN,
    };
    pub const GLP: Country = Country {
        name: "Guadeloupe",
        iso2: Iso2::GP,
        iso3: Iso3::GLP,
        top_level_domain: TopLevelDomain::GP,
    };
    pub const GMB: Country = Country {
        name: "Gambia",
        iso2: Iso2::GM,
        iso3: Iso3::GMB,
        top_level_domain: TopLevelDomain::GM,
    };
    pub const GNB: Country = Country {
        name: "Guinea - Bissau",
        iso2: Iso2::GW,
        iso3: Iso3::GNB,
        top_level_domain: TopLevelDomain::GW,
    };
    pub const GNQ: Country = Country {
        name: "Equatorial Guinea",
        iso2: Iso2::GQ,
        iso3: Iso3::GNQ,
        top_level_domain: TopLevelDomain::GQ,
    };
    pub const GRC: Country = Country {
        name: "Greece",
        iso2: Iso2::GR,
        iso3: Iso3::GRC,
        top_level_domain: TopLevelDomain::GR,
    };
    pub const GRD: Country = Country {
        name: "Grenada",
        iso2: Iso2::GD,
        iso3: Iso3::GRD,
        top_level_domain: TopLevelDomain::GD,
    };
    pub const GRL: Country = Country {
        name: "Greenland",
        iso2: Iso2::GL,
        iso3: Iso3::GRL,
        top_level_domain: TopLevelDomain::GL,
    };
    pub const GTM: Country = Country {
        name: "Guatemala",
        iso2: Iso2::GT,
        iso3: Iso3::GTM,
        top_level_domain: TopLevelDomain::GT,
    };
    pub const GUF: Country = Country {
        name: "French Guiana",
        iso2: Iso2::GF,
        iso3: Iso3::GUF,
        top_level_domain: TopLevelDomain::GF,
    };
    pub const GUM: Country = Country {
        name: "Guam",
        iso2: Iso2::GU,
        iso3: Iso3::GUM,
        top_level_domain: TopLevelDomain::GU,
    };
    pub const GUY: Country = Country {
        name: "Guyana",
        iso2: Iso2::GY,
        iso3: Iso3::GUY,
        top_level_domain: TopLevelDomain::GY,
    };

    pub const HKG: Country = Country {
        name: "Hong Kong",
        iso2: Iso2::HK,
        iso3: Iso3::HKG,
        top_level_domain: TopLevelDomain::HK,
    };
    pub const HMD: Country = Country {
        name: "Heard Island and McDonald Islands",
        iso2: Iso2::HM,
        iso3: Iso3::HMD,
        top_level_domain: TopLevelDomain::HM,
    };
    pub const HND: Country = Country {
        name: "Honduras",
        iso2: Iso2::HN,
        iso3: Iso3::HND,
        top_level_domain: TopLevelDomain::HN,
    };
    pub const HRV: Country = Country {
        name: "Croatia",
        iso2: Iso2::HR,
        iso3: Iso3::HRV,
        top_level_domain: TopLevelDomain::HR,
    };
    pub const HTI: Country = Country {
        name: "Haiti",
        iso2: Iso2::HT,
        iso3: Iso3::HTI,
        top_level_domain: TopLevelDomain::HT,
    };
    pub const HUN: Country = Country {
        name: "Hungary",
        iso2: Iso2::HU,
        iso3: Iso3::HUN,
        top_level_domain: TopLevelDomain::HU,
    };

    pub const IDN: Country = Country {
        name: "Indonesia",
        iso2: Iso2::ID,
        iso3: Iso3::IDN,
        top_level_domain: TopLevelDomain::ID,
    };
    pub const IMN: Country = Country {
        name: "Isle of Man",
        iso2: Iso2::IM,
        iso3: Iso3::IMN,
        top_level_domain: TopLevelDomain::IM,
    };
    pub const IND: Country = Country {
        name: "India",
        iso2: Iso2::IN,
        iso3: Iso3::IND,
        top_level_domain: TopLevelDomain::IN,
    };
    pub const IOT: Country = Country {
        name: "British Indian Ocean Territory",
        iso2: Iso2::IO,
        iso3: Iso3::IOT,
        top_level_domain: TopLevelDomain::IO,
    };
    pub const IRL: Country = Country {
        name: "Ireland",
        iso2: Iso2::IE,
        iso3: Iso3::IRL,
        top_level_domain: TopLevelDomain::IE,
    };
    pub const IRN: Country = Country {
        name: "Iran",
        iso2: Iso2::IR,
        iso3: Iso3::IRN,
        top_level_domain: TopLevelDomain::IR,
    };
    pub const IRQ: Country = Country {
        name: "Iraq",
        iso2: Iso2::IQ,
        iso3: Iso3::IRQ,
        top_level_domain: TopLevelDomain::IQ,
    };
    pub const ISL: Country = Country {
        name: "Iceland",
        iso2: Iso2::IS,
        iso3: Iso3::ISL,
        top_level_domain: TopLevelDomain::IS,
    };
    pub const ISR: Country = Country {
        name: "Israel",
        iso2: Iso2::IL,
        iso3: Iso3::ISR,
        top_level_domain: TopLevelDomain::IL,
    };
    pub const ITA: Country = Country {
        name: "Italy",
        iso2: Iso2::IT,
        iso3: Iso3::ITA,
        top_level_domain: TopLevelDomain::IT,
    };

    pub const JAM: Country = Country {
        name: "Jamaica",
        iso2: Iso2::JM,
        iso3: Iso3::JAM,
        top_level_domain: TopLevelDomain::JM,
    };
    pub const JEY: Country = Country {
        name: "Jersey",
        iso2: Iso2::JE,
        iso3: Iso3::JEY,
        top_level_domain: TopLevelDomain::JE,
    };
    pub const JOR: Country = Country {
        name: "Jordan",
        iso2: Iso2::JO,
        iso3: Iso3::JOR,
        top_level_domain: TopLevelDomain::JO,
    };
    pub const JPN: Country = Country {
        name: "Japan",
        iso2: Iso2::JP,
        iso3: Iso3::JPN,
        top_level_domain: TopLevelDomain::JP,
    };

    pub const KAZ: Country = Country {
        name: "Kazakhstan",
        iso2: Iso2::KZ,
        iso3: Iso3::KAZ,
        top_level_domain: TopLevelDomain::KZ,
    };
    pub const KEN: Country = Country {
        name: "Kenya",
        iso2: Iso2::KE,
        iso3: Iso3::KEN,
        top_level_domain: TopLevelDomain::KE,
    };
    pub const KGZ: Country = Country {
        name: "Kyrgyzstan",
        iso2: Iso2::KG,
        iso3: Iso3::KGZ,
        top_level_domain: TopLevelDomain::KG,
    };
    pub const KHM: Country = Country {
        name: "Cambodia",
        iso2: Iso2::KH,
        iso3: Iso3::KHM,
        top_level_domain: TopLevelDomain::KH,
    };
    pub const KIR: Country = Country {
        name: "Kiribati",
        iso2: Iso2::KI,
        iso3: Iso3::KIR,
        top_level_domain: TopLevelDomain::KI,
    };
    pub const KNA: Country = Country {
        name: "Saint Kitts and Nevis",
        iso2: Iso2::KN,
        iso3: Iso3::KNA,
        top_level_domain: TopLevelDomain::KN,
    };
    pub const KOR: Country = Country {
        name: "South Korea",
        iso2: Iso2::KR,
        iso3: Iso3::KOR,
        top_level_domain: TopLevelDomain::KR,
    };
    pub const KWT: Country = Country {
        name: "Kuwait",
        iso2: Iso2::KW,
        iso3: Iso3::KWT,
        top_level_domain: TopLevelDomain::KW,
    };

    pub const LAO: Country = Country {
        name: "Laos",
        iso2: Iso2::LA,
        iso3: Iso3::LAO,
        top_level_domain: TopLevelDomain::LA,
    };
    pub const LBN: Country = Country {
        name: "Lebanon",
        iso2: Iso2::LB,
        iso3: Iso3::LBN,
        top_level_domain: TopLevelDomain::LB,
    };
    pub const LBR: Country = Country {
        name: "Liberia",
        iso2: Iso2::LR,
        iso3: Iso3::LBR,
        top_level_domain: TopLevelDomain::LR,
    };
    pub const LBY: Country = Country {
        name: "Libya",
        iso2: Iso2::LY,
        iso3: Iso3::LBY,
        top_level_domain: TopLevelDomain::LY,
    };
    pub const LCA: Country = Country {
        name: "Saint Lucia",
        iso2: Iso2::LC,
        iso3: Iso3::LCA,
        top_level_domain: TopLevelDomain::LC,
    };
    pub const LIE: Country = Country {
        name: "Liechtenstein",
        iso2: Iso2::LI,
        iso3: Iso3::LIE,
        top_level_domain: TopLevelDomain::LI,
    };
    pub const LKA: Country = Country {
        name: "Sri Lanka",
        iso2: Iso2::LK,
        iso3: Iso3::LKA,
        top_level_domain: TopLevelDomain::LK,
    };
    pub const LSO: Country = Country {
        name: "Lesotho",
        iso2: Iso2::LS,
        iso3: Iso3::LSO,
        top_level_domain: TopLevelDomain::LS,
    };
    pub const LTU: Country = Country {
        name: "Lithuania",
        iso2: Iso2::LT,
        iso3: Iso3::LTU,
        top_level_domain: TopLevelDomain::LT,
    };
    pub const LUX: Country = Country {
        name: "Luxembourg",
        iso2: Iso2::LU,
        iso3: Iso3::LUX,
        top_level_domain: TopLevelDomain::LU,
    };
    pub const LVA: Country = Country {
        name: "LVA",
        iso2: Iso2::LV,
        iso3: Iso3::LVA,
        top_level_domain: TopLevelDomain::LV,
    };

    pub const MAC: Country = Country {
        name: "Macao",
        iso2: Iso2::MO,
        iso3: Iso3::MAC,
        top_level_domain: TopLevelDomain::MO,
    };
    pub const MAF: Country = Country {
        name: "Saint Martin",
        iso2: Iso2::MF,
        iso3: Iso3::MAF,
        top_level_domain: TopLevelDomain::MF,
    };
    pub const MAR: Country = Country {
        name: "Morocco",
        iso2: Iso2::MA,
        iso3: Iso3::MAR,
        top_level_domain: TopLevelDomain::MA,
    };
    pub const MCO: Country = Country {
        name: "Monaco",
        iso2: Iso2::MC,
        iso3: Iso3::MCO,
        top_level_domain: TopLevelDomain::MC,
    };
    pub const MDA: Country = Country {
        name: "Moldova",
        iso2: Iso2::MD,
        iso3: Iso3::MDA,
        top_level_domain: TopLevelDomain::MD,
    };
    pub const MDG: Country = Country {
        name: "Madagascar",
        iso2: Iso2::MG,
        iso3: Iso3::MDG,
        top_level_domain: TopLevelDomain::MG,
    };
    pub const MDV: Country = Country {
        name: "Maldives",
        iso2: Iso2::MV,
        iso3: Iso3::MDV,
        top_level_domain: TopLevelDomain::MV,
    };
    pub const MEX: Country = Country {
        name: "Mexico",
        iso2: Iso2::MX,
        iso3: Iso3::MEX,
        top_level_domain: TopLevelDomain::MX,
    };
    pub const MHL: Country = Country {
        name: "Marshall Islands",
        iso2: Iso2::MH,
        iso3: Iso3::MHL,
        top_level_domain: TopLevelDomain::MH,
    };
    pub const MID: Country = Country {
        name: "Midway Islands",
        iso2: Iso2::MI,
        iso3: Iso3::MID,
        top_level_domain: TopLevelDomain::MI,
    };
    pub const MKD: Country = Country {
        name: "North Macedonia",
        iso2: Iso2::MK,
        iso3: Iso3::MKD,
        top_level_domain: TopLevelDomain::MK,
    };
    pub const MLI: Country = Country {
        name: "Mali",
        iso2: Iso2::ML,
        iso3: Iso3::MLI,
        top_level_domain: TopLevelDomain::ML,
    };
    pub const MLT: Country = Country {
        name: "Malta",
        iso2: Iso2::MT,
        iso3: Iso3::MLT,
        top_level_domain: TopLevelDomain::MT,
    };
    pub const MMR: Country = Country {
        name: "Myanmar",
        iso2: Iso2::MM,
        iso3: Iso3::MMR,
        top_level_domain: TopLevelDomain::MM,
    };
    pub const MNE: Country = Country {
        name: "Montenegro",
        iso2: Iso2::ME,
        iso3: Iso3::MNE,
        top_level_domain: TopLevelDomain::ME,
    };
    pub const MNG: Country = Country {
        name: "Mongolia",
        iso2: Iso2::MN,
        iso3: Iso3::MNG,
        top_level_domain: TopLevelDomain::MN,
    };
    pub const MNP: Country = Country {
        name: "Northern Mariana Islands",
        iso2: Iso2::MP,
        iso3: Iso3::MNP,
        top_level_domain: TopLevelDomain::MP,
    };
    pub const MOZ: Country = Country {
        name: "Mozambique",
        iso2: Iso2::MZ,
        iso3: Iso3::MOZ,
        top_level_domain: TopLevelDomain::MZ,
    };
    pub const MRT: Country = Country {
        name: "Mauritania",
        iso2: Iso2::MR,
        iso3: Iso3::MRT,
        top_level_domain: TopLevelDomain::MR,
    };
    pub const MSR: Country = Country {
        name: "Montserrat",
        iso2: Iso2::MS,
        iso3: Iso3::MSR,
        top_level_domain: TopLevelDomain::MS,
    };
    pub const MTQ: Country = Country {
        name: "Martinique",
        iso2: Iso2::MQ,
        iso3: Iso3::MTQ,
        top_level_domain: TopLevelDomain::MQ,
    };
    pub const MUS: Country = Country {
        name: "Mauritius",
        iso2: Iso2::MU,
        iso3: Iso3::MUS,
        top_level_domain: TopLevelDomain::MU,
    };
    pub const MWI: Country = Country {
        name: "Malawi",
        iso2: Iso2::MW,
        iso3: Iso3::MWI,
        top_level_domain: TopLevelDomain::MW,
    };

    pub const MYS: Country = Country {
        name: "Malaysia",
        iso2: Iso2::MY,
        iso3: Iso3::MYS,
        top_level_domain: TopLevelDomain::MY,
    };
    pub const MYT: Country = Country {
        name: "Mayotte",
        iso2: Iso2::YT,
        iso3: Iso3::MYT,
        top_level_domain: TopLevelDomain::YT,
    };

    pub const NAM: Country = Country {
        name: "Namibia",
        iso2: Iso2::NA,
        iso3: Iso3::NAM,
        top_level_domain: TopLevelDomain::NA,
    };
    pub const NCL: Country = Country {
        name: "New Caledonia",
        iso2: Iso2::NC,
        iso3: Iso3::NCL,
        top_level_domain: TopLevelDomain::NC,
    };
    pub const NER: Country = Country {
        name: "Niger",
        iso2: Iso2::NE,
        iso3: Iso3::NER,
        top_level_domain: TopLevelDomain::NE,
    };
    pub const NFK: Country = Country {
        name: "Norfolk Island",
        iso2: Iso2::NF,
        iso3: Iso3::NFK,
        top_level_domain: TopLevelDomain::NF,
    };
    pub const NGA: Country = Country {
        name: "Nigeria",
        iso2: Iso2::NG,
        iso3: Iso3::NGA,
        top_level_domain: TopLevelDomain::NG,
    };
    pub const NIC: Country = Country {
        name: "Nicaragua",
        iso2: Iso2::NI,
        iso3: Iso3::NIC,
        top_level_domain: TopLevelDomain::NI,
    };
    pub const NIU: Country = Country {
        name: "Niue",
        iso2: Iso2::NU,
        iso3: Iso3::NIU,
        top_level_domain: TopLevelDomain::NU,
    };
    pub const NLD: Country = Country {
        name: "Netherlands",
        iso2: Iso2::NL,
        iso3: Iso3::NLD,
        top_level_domain: TopLevelDomain::NL,
    };
    pub const NOR: Country = Country {
        name: "Norway",
        iso2: Iso2::NO,
        iso3: Iso3::NOR,
        top_level_domain: TopLevelDomain::NO,
    };
    pub const NPL: Country = Country {
        name: "Nepal",
        iso2: Iso2::NP,
        iso3: Iso3::NPL,
        top_level_domain: TopLevelDomain::NP,
    };
    pub const NRU: Country = Country {
        name: "Nauru",
        iso2: Iso2::NR,
        iso3: Iso3::NRU,
        top_level_domain: TopLevelDomain::NR,
    };
    pub const NZL: Country = Country {
        name: "New Zealand",
        iso2: Iso2::NZ,
        iso3: Iso3::NZL,
        top_level_domain: TopLevelDomain::NZ,
    };

    pub const OMN: Country = Country {
        name: "Oman",
        iso2: Iso2::OM,
        iso3: Iso3::OMN,
        top_level_domain: TopLevelDomain::OM,
    };

    pub const PAK: Country = Country {
        name: "Pakistan",
        iso2: Iso2::PK,
        iso3: Iso3::PAK,
        top_level_domain: TopLevelDomain::PK,
    };
    pub const PAN: Country = Country {
        name: "Panama",
        iso2: Iso2::PA,
        iso3: Iso3::PAN,
        top_level_domain: TopLevelDomain::PA,
    };
    pub const PCN: Country = Country {
        name: "Pitcairn",
        iso2: Iso2::PN,
        iso3: Iso3::PCN,
        top_level_domain: TopLevelDomain::PN,
    };
    pub const PER: Country = Country {
        name: "Peru",
        iso2: Iso2::PE,
        iso3: Iso3::PER,
        top_level_domain: TopLevelDomain::PE,
    };
    pub const PHL: Country = Country {
        name: "Philippines",
        iso2: Iso2::PH,
        iso3: Iso3::PHL,
        top_level_domain: TopLevelDomain::PH,
    };
    pub const PLW: Country = Country {
        name: "Palau",
        iso2: Iso2::PW,
        iso3: Iso3::PLW,
        top_level_domain: TopLevelDomain::PW,
    };
    pub const PNG: Country = Country {
        name: "Papua New Guinea",
        iso2: Iso2::PG,
        iso3: Iso3::PNG,
        top_level_domain: TopLevelDomain::PG,
    };
    pub const POL: Country = Country {
        name: "Poland",
        iso2: Iso2::PL,
        iso3: Iso3::POL,
        top_level_domain: TopLevelDomain::PL,
    };
    pub const PRI: Country = Country {
        name: "Puerto Rico",
        iso2: Iso2::PR,
        iso3: Iso3::PRI,
        top_level_domain: TopLevelDomain::PR,
    };
    pub const PRK: Country = Country {
        name: "North Korea",
        iso2: Iso2::KP,
        iso3: Iso3::PRK,
        top_level_domain: TopLevelDomain::KP,
    };
    pub const PRT: Country = Country {
        name: "Portugal",
        iso2: Iso2::PT,
        iso3: Iso3::PRT,
        top_level_domain: TopLevelDomain::PT,
    };
    pub const PRY: Country = Country {
        name: "Paraguay",
        iso2: Iso2::PY,
        iso3: Iso3::PRY,
        top_level_domain: TopLevelDomain::PY,
    };
    pub const PSE: Country = Country {
        name: "Palestine",
        iso2: Iso2::PS,
        iso3: Iso3::PSE,
        top_level_domain: TopLevelDomain::PS,
    };
    pub const PYF: Country = Country {
        name: "French Polynesia",
        iso2: Iso2::PF,
        iso3: Iso3::PYF,
        top_level_domain: TopLevelDomain::PF,
    };

    pub const QAT: Country = Country {
        name: "Qatar",
        iso2: Iso2::QA,
        iso3: Iso3::QAT,
        top_level_domain: TopLevelDomain::QA,
    };

    pub const REU: Country = Country {
        name: "Réunion Island",
        iso2: Iso2::RE,
        iso3: Iso3::REU,
        top_level_domain: TopLevelDomain::RE,
    };
    pub const ROU: Country = Country {
        name: "Romania",
        iso2: Iso2::RO,
        iso3: Iso3::ROU,
        top_level_domain: TopLevelDomain::RO,
    };
    pub const RUS: Country = Country {
        name: "Russia",
        iso2: Iso2::RU,
        iso3: Iso3::RUS,
        top_level_domain: TopLevelDomain::RU,
    };
    pub const RWA: Country = Country {
        name: "Rwanda",
        iso2: Iso2::RW,
        iso3: Iso3::RWA,
        top_level_domain: TopLevelDomain::RW,
    };

    pub const SAU: Country = Country {
        name: "Saudi Arabia",
        iso2: Iso2::SA,
        iso3: Iso3::SAU,
        top_level_domain: TopLevelDomain::SA,
    };
    pub const SDN: Country = Country {
        name: "Sudan",
        iso2: Iso2::SD,
        iso3: Iso3::SDN,
        top_level_domain: TopLevelDomain::SD,
    };
    pub const SEN: Country = Country {
        name: "Senegal",
        iso2: Iso2::SN,
        iso3: Iso3::SEN,
        top_level_domain: TopLevelDomain::SN,
    };
    pub const SGP: Country = Country {
        name: "Singapore",
        iso2: Iso2::SG,
        iso3: Iso3::SGP,
        top_level_domain: TopLevelDomain::SG,
    };
    pub const SGS: Country = Country {
        name: "South Georgia and the South Sandwich Islands",
        iso2: Iso2::GS,
        iso3: Iso3::SGS,
        top_level_domain: TopLevelDomain::GS,
    };
    pub const SHN: Country = Country {
        name: "Saint Helena, Ascension and Tristan da Cunha",
        iso2: Iso2::SH,
        iso3: Iso3::SHN,
        top_level_domain: TopLevelDomain::SH,
    };
    pub const SJM: Country = Country {
        name: "Svalbard and Jan Mayen",
        iso2: Iso2::SJ,
        iso3: Iso3::SJM,
        top_level_domain: TopLevelDomain::SJ,
    };
    pub const SLB: Country = Country {
        name: "Solomon Islands",
        iso2: Iso2::SB,
        iso3: Iso3::SLB,
        top_level_domain: TopLevelDomain::SB,
    };
    pub const SLE: Country = Country {
        name: "Sierra Leone",
        iso2: Iso2::SL,
        iso3: Iso3::SLE,
        top_level_domain: TopLevelDomain::SL,
    };
    pub const SLV: Country = Country {
        name: "El Salvador",
        iso2: Iso2::SV,
        iso3: Iso3::SLV,
        top_level_domain: TopLevelDomain::SV,
    };
    pub const SMR: Country = Country {
        name: "San Marino",
        iso2: Iso2::SM,
        iso3: Iso3::SMR,
        top_level_domain: TopLevelDomain::SM,
    };
    pub const SOM: Country = Country {
        name: "Somalia",
        iso2: Iso2::SO,
        iso3: Iso3::SOM,
        top_level_domain: TopLevelDomain::SO,
    };
    pub const SPM: Country = Country {
        name: "Saint Pierre and Miquelon",
        iso2: Iso2::PM,
        iso3: Iso3::SPM,
        top_level_domain: TopLevelDomain::PM,
    };
    pub const SRB: Country = Country {
        name: "Serbia",
        iso2: Iso2::RS,
        iso3: Iso3::SRB,
        top_level_domain: TopLevelDomain::RS,
    };
    pub const SSD: Country = Country {
        name: "South Sudan",
        iso2: Iso2::SS,
        iso3: Iso3::SSD,
        top_level_domain: TopLevelDomain::SS,
    };
    pub const STP: Country = Country {
        name: "Sao Tome and Principe",
        iso2: Iso2::ST,
        iso3: Iso3::STP,
        top_level_domain: TopLevelDomain::ST,
    };
    pub const SUR: Country = Country {
        name: "Suriname",
        iso2: Iso2::SR,
        iso3: Iso3::SUR,
        top_level_domain: TopLevelDomain::SR,
    };
    pub const SVK: Country = Country {
        name: "Slovakia",
        iso2: Iso2::SK,
        iso3: Iso3::SVK,
        top_level_domain: TopLevelDomain::SK,
    };
    pub const SVN: Country = Country {
        name: "Slovenia",
        iso2: Iso2::SI,
        iso3: Iso3::SVN,
        top_level_domain: TopLevelDomain::SI,
    };
    pub const SWE: Country = Country {
        name: "Sweden",
        iso2: Iso2::SE,
        iso3: Iso3::SWE,
        top_level_domain: TopLevelDomain::SE,
    };
    pub const SWZ: Country = Country {
        name: "Swaziland",
        iso2: Iso2::SZ,
        iso3: Iso3::SWZ,
        top_level_domain: TopLevelDomain::SZ,
    };
    pub const SXM: Country = Country {
        name: "Sint Maarten",
        iso2: Iso2::SX,
        iso3: Iso3::SXM,
        top_level_domain: TopLevelDomain::SX,
    };
    pub const SYC: Country = Country {
        name: "Seychelles",
        iso2: Iso2::SC,
        iso3: Iso3::SYC,
        top_level_domain: TopLevelDomain::SC,
    };
    pub const SYR: Country = Country {
        name: "Syria",
        iso2: Iso2::SY,
        iso3: Iso3::SYR,
        top_level_domain: TopLevelDomain::SY,
    };

    pub const TCA: Country = Country {
        name: "Turks and Caicos Islands",
        iso2: Iso2::TC,
        iso3: Iso3::TCA,
        top_level_domain: TopLevelDomain::TC,
    };
    pub const TCD: Country = Country {
        name: "Chad",
        iso2: Iso2::TD,
        iso3: Iso3::TCD,
        top_level_domain: TopLevelDomain::TD,
    };
    pub const TGO: Country = Country {
        name: "Togo",
        iso2: Iso2::TG,
        iso3: Iso3::TGO,
        top_level_domain: TopLevelDomain::TG,
    };
    pub const THA: Country = Country {
        name: "Thailand",
        iso2: Iso2::TH,
        iso3: Iso3::THA,
        top_level_domain: TopLevelDomain::TH,
    };
    pub const TJK: Country = Country {
        name: "Tajikistan",
        iso2: Iso2::TJ,
        iso3: Iso3::TJK,
        top_level_domain: TopLevelDomain::TJ,
    };
    pub const TKL: Country = Country {
        name: "Tokelau",
        iso2: Iso2::TK,
        iso3: Iso3::TKL,
        top_level_domain: TopLevelDomain::TK,
    };
    pub const TKM: Country = Country {
        name: "Turkmenistan",
        iso2: Iso2::TM,
        iso3: Iso3::TKM,
        top_level_domain: TopLevelDomain::TM,
    };
    pub const TLS: Country = Country {
        name: "Timor - Leste",
        iso2: Iso2::TL,
        iso3: Iso3::TLS,
        top_level_domain: TopLevelDomain::TL,
    };
    pub const TON: Country = Country {
        name: "Tonga",
        iso2: Iso2::TO,
        iso3: Iso3::TON,
        top_level_domain: TopLevelDomain::TO,
    };
    pub const TTO: Country = Country {
        name: "Trinidad and Tobago",
        iso2: Iso2::TT,
        iso3: Iso3::TTO,
        top_level_domain: TopLevelDomain::TT,
    };
    pub const TUN: Country = Country {
        name: "Tunisia",
        iso2: Iso2::TN,
        iso3: Iso3::TUN,
        top_level_domain: TopLevelDomain::TN,
    };
    pub const TUR: Country = Country {
        name: "Turkey",
        iso2: Iso2::TR,
        iso3: Iso3::TUR,
        top_level_domain: TopLevelDomain::TR,
    };
    pub const TUV: Country = Country {
        name: "Tuvalu",
        iso2: Iso2::TV,
        iso3: Iso3::TUV,
        top_level_domain: TopLevelDomain::TV,
    };
    pub const TWN: Country = Country {
        name: "Taiwan",
        iso2: Iso2::TW,
        iso3: Iso3::TWN,
        top_level_domain: TopLevelDomain::TW,
    };

    pub const TZA: Country = Country {
        name: "Tanzania",
        iso2: Iso2::TZ,
        iso3: Iso3::TZA,
        top_level_domain: TopLevelDomain::TZ,
    };

    pub const UGA: Country = Country {
        name: "Uganda",
        iso2: Iso2::UG,
        iso3: Iso3::UGA,
        top_level_domain: TopLevelDomain::UG,
    };
    pub const UKR: Country = Country {
        name: "Ukraine",
        iso2: Iso2::UA,
        iso3: Iso3::UKR,
        top_level_domain: TopLevelDomain::UA,
    };
    pub const UMI: Country = Country {
        name: "United States Minor Outlying Islands",
        iso2: Iso2::UM,
        iso3: Iso3::UMI,
        top_level_domain: TopLevelDomain::UM,
    };
    pub const URY: Country = Country {
        name: "Uruguay",
        iso2: Iso2::UY,
        iso3: Iso3::URY,
        top_level_domain: TopLevelDomain::UY,
    };
    pub const USA: Country = Country {
        name: "United States",
        iso2: Iso2::US,
        iso3: Iso3::USA,
        top_level_domain: TopLevelDomain::US,
    };
    pub const UZB: Country = Country {
        name: "Uzbekistan",
        iso2: Iso2::UZ,
        iso3: Iso3::UZB,
        top_level_domain: TopLevelDomain::UZ,
    };

    pub const VAT: Country = Country {
        name: "Holy See",
        iso2: Iso2::VA,
        iso3: Iso3::VAT,
        top_level_domain: TopLevelDomain::VA,
    };
    pub const VCT: Country = Country {
        name: "Saint Vincent and the Grenadines",
        iso2: Iso2::VC,
        iso3: Iso3::VCT,
        top_level_domain: TopLevelDomain::VC,
    };
    pub const VEN: Country = Country {
        name: "Venezuela",
        iso2: Iso2::VE,
        iso3: Iso3::VEN,
        top_level_domain: TopLevelDomain::VE,
    };
    pub const VGB: Country = Country {
        name: "British Virgin Islands",
        iso2: Iso2::VG,
        iso3: Iso3::VGB,
        top_level_domain: TopLevelDomain::VG,
    };
    pub const VIR: Country = Country {
        name: "U.S. Virgin Islands",
        iso2: Iso2::VI,
        iso3: Iso3::VIR,
        top_level_domain: TopLevelDomain::VI,
    };
    pub const VNM: Country = Country {
        name: "Vietnam",
        iso2: Iso2::VN,
        iso3: Iso3::VNM,
        top_level_domain: TopLevelDomain::VN,
    };
    pub const VUT: Country = Country {
        name: "Vanuatu",
        iso2: Iso2::VU,
        iso3: Iso3::VUT,
        top_level_domain: TopLevelDomain::VU,
    };

    pub const WLF: Country = Country {
        name: "Wallis and Futuna",
        iso2: Iso2::WF,
        iso3: Iso3::WLF,
        top_level_domain: TopLevelDomain::WF,
    };
    pub const WSM: Country = Country {
        name: "Samoa",
        iso2: Iso2::WS,
        iso3: Iso3::WSM,
        top_level_domain: TopLevelDomain::WS,
    };

    pub const YEM: Country = Country {
        name: "Yemen",
        iso2: Iso2::YE,
        iso3: Iso3::YEM,
        top_level_domain: TopLevelDomain::YE,
    };

    pub const ZAF: Country = Country {
        name: "South Africa",
        iso2: Iso2::ZA,
        iso3: Iso3::ZAF,
        top_level_domain: TopLevelDomain::ZA,
    };
    pub const ZMB: Country = Country {
        name: "Zambia",
        iso2: Iso2::ZM,
        iso3: Iso3::ZMB,
        top_level_domain: TopLevelDomain::ZM,
    };
    pub const ZWE: Country = Country {
        name: "Zimbabwe",
        iso2: Iso2::ZW,
        iso3: Iso3::ZWE,
        top_level_domain: TopLevelDomain::ZW,
    };
}

impl Country {
    pub fn iter() -> Iter<'static, Country> {
        ALL_COUNTRIES.iter()
    }
}
