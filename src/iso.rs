use crate::models::country::{Country, NUMBER_OF_COUNTRIES};
use crate::models::iso::{Iso2, Iso3};
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref ISO2_MAP: HashMap<Iso2, Country> = {
        let mut map = HashMap::with_capacity(NUMBER_OF_COUNTRIES.into());
        map.insert(Iso2::AD, Country::AND);
        map.insert(Iso2::AE, Country::ARE);
        map.insert(Iso2::AF, Country::AFG);
        map.insert(Iso2::AG, Country::ATG);
        map.insert(Iso2::AI, Country::AIA);
        map.insert(Iso2::AL, Country::ALB);
        map.insert(Iso2::AM, Country::ARM);
        map.insert(Iso2::AN, Country::ANT);
        map.insert(Iso2::AO, Country::AGO);
        map.insert(Iso2::AQ, Country::ATA);
        map.insert(Iso2::AR, Country::ARG);
        map.insert(Iso2::AS, Country::ASM);
        map.insert(Iso2::AT, Country::AUT);
        map.insert(Iso2::AU, Country::AUS);
        map.insert(Iso2::AW, Country::ABW);
        map.insert(Iso2::AX, Country::ALA);
        map.insert(Iso2::AZ, Country::AZE);

        map.insert(Iso2::BA, Country::BIH);
        map.insert(Iso2::BB, Country::BRB);
        map.insert(Iso2::BD, Country::BGD);
        map.insert(Iso2::BE, Country::BEL);
        map.insert(Iso2::BF, Country::BFA);
        map.insert(Iso2::BG, Country::BGR);
        map.insert(Iso2::BH, Country::BHR);
        map.insert(Iso2::BI, Country::BDI);
        map.insert(Iso2::BJ, Country::BEN);
        map.insert(Iso2::BL, Country::BLM);
        map.insert(Iso2::BM, Country::BMU);
        map.insert(Iso2::BN, Country::BRN);
        map.insert(Iso2::BO, Country::BOL);
        map.insert(Iso2::BQ, Country::BES);
        map.insert(Iso2::BR, Country::BRA);
        map.insert(Iso2::BS, Country::BHS);
        map.insert(Iso2::BT, Country::BTN);
        map.insert(Iso2::BV, Country::BVT);
        map.insert(Iso2::BW, Country::BWA);
        map.insert(Iso2::BY, Country::BLR);
        map.insert(Iso2::BZ, Country::BLZ);

        map.insert(Iso2::CA, Country::CAN);
        map.insert(Iso2::CC, Country::CCK);
        map.insert(Iso2::CD, Country::COD);
        map.insert(Iso2::CF, Country::CAF);
        map.insert(Iso2::CG, Country::COG);
        map.insert(Iso2::CH, Country::CHE);
        map.insert(Iso2::CI, Country::CIV);
        map.insert(Iso2::CK, Country::COK);
        map.insert(Iso2::CL, Country::CHL);
        map.insert(Iso2::CM, Country::CMR);
        map.insert(Iso2::CN, Country::CHN);
        map.insert(Iso2::CO, Country::COL);
        map.insert(Iso2::CR, Country::CRI);
        map.insert(Iso2::CU, Country::CUB);
        map.insert(Iso2::CW, Country::CUW);
        map.insert(Iso2::CV, Country::CPV);
        map.insert(Iso2::CX, Country::CXR);
        map.insert(Iso2::CY, Country::CYP);
        map.insert(Iso2::CZ, Country::CZE);

        map.insert(Iso2::DE, Country::DEU);
        map.insert(Iso2::DJ, Country::DJI);
        map.insert(Iso2::DK, Country::DNK);
        map.insert(Iso2::DM, Country::DMA);
        map.insert(Iso2::DO, Country::DOM);
        map.insert(Iso2::DZ, Country::DZA);

        map.insert(Iso2::EC, Country::ECU);
        map.insert(Iso2::EE, Country::EST);
        map.insert(Iso2::EG, Country::EGY);
        map.insert(Iso2::EH, Country::ESH);
        map.insert(Iso2::ER, Country::ERI);
        map.insert(Iso2::ES, Country::ESP);
        map.insert(Iso2::ET, Country::ETH);

        map.insert(Iso2::FI, Country::FIN);
        map.insert(Iso2::FJ, Country::FJI);
        map.insert(Iso2::FK, Country::FLK);
        map.insert(Iso2::FM, Country::FSM);
        map.insert(Iso2::FO, Country::FRO);
        map.insert(Iso2::FR, Country::FRA);

        map.insert(Iso2::GA, Country::GAB);
        map.insert(Iso2::GB, Country::GBR);
        map.insert(Iso2::GD, Country::GRD);
        map.insert(Iso2::GE, Country::GEO);
        map.insert(Iso2::GF, Country::GUF);
        map.insert(Iso2::GG, Country::GGY);
        map.insert(Iso2::GH, Country::GHA);
        map.insert(Iso2::GI, Country::GIB);
        map.insert(Iso2::GL, Country::GRL);
        map.insert(Iso2::GM, Country::GMB);
        map.insert(Iso2::GN, Country::GIN);
        map.insert(Iso2::GP, Country::GLP);
        map.insert(Iso2::GQ, Country::GNQ);
        map.insert(Iso2::GR, Country::GRC);
        map.insert(Iso2::GS, Country::SGS);
        map.insert(Iso2::GT, Country::GTM);
        map.insert(Iso2::GU, Country::GUM);
        map.insert(Iso2::GW, Country::GNB);
        map.insert(Iso2::GY, Country::GUY);

        map.insert(Iso2::HK, Country::HKG);
        map.insert(Iso2::HM, Country::HMD);
        map.insert(Iso2::HN, Country::HND);
        map.insert(Iso2::HR, Country::HRV);
        map.insert(Iso2::HT, Country::HTI);
        map.insert(Iso2::HU, Country::HUN);

        map.insert(Iso2::ID, Country::IDN);
        map.insert(Iso2::IE, Country::IRL);
        map.insert(Iso2::IL, Country::ISR);
        map.insert(Iso2::IM, Country::IMN);
        map.insert(Iso2::IN, Country::IND);
        map.insert(Iso2::IO, Country::IOT);
        map.insert(Iso2::IQ, Country::IRQ);
        map.insert(Iso2::IR, Country::IRN);
        map.insert(Iso2::IS, Country::ISL);
        map.insert(Iso2::IT, Country::ITA);

        map.insert(Iso2::JE, Country::JEY);
        map.insert(Iso2::JM, Country::JAM);
        map.insert(Iso2::JO, Country::JOR);
        map.insert(Iso2::JP, Country::JPN);

        map.insert(Iso2::KE, Country::KEN);
        map.insert(Iso2::KG, Country::KGZ);
        map.insert(Iso2::KH, Country::KHM);
        map.insert(Iso2::KI, Country::KIR);
        map.insert(Iso2::KM, Country::COM);
        map.insert(Iso2::KN, Country::KNA);
        map.insert(Iso2::KP, Country::PRK);
        map.insert(Iso2::KR, Country::KOR);
        map.insert(Iso2::KW, Country::KWT);
        map.insert(Iso2::KY, Country::CYM);
        map.insert(Iso2::KZ, Country::KAZ);

        map.insert(Iso2::LA, Country::LAO);
        map.insert(Iso2::LB, Country::LBN);
        map.insert(Iso2::LC, Country::LCA);
        map.insert(Iso2::LI, Country::LIE);
        map.insert(Iso2::LK, Country::LKA);
        map.insert(Iso2::LR, Country::LBR);
        map.insert(Iso2::LS, Country::LSO);
        map.insert(Iso2::LT, Country::LTU);
        map.insert(Iso2::LU, Country::LUX);
        map.insert(Iso2::LV, Country::LVA);
        map.insert(Iso2::LY, Country::LBY);

        map.insert(Iso2::MA, Country::MAR);
        map.insert(Iso2::MC, Country::MCO);
        map.insert(Iso2::MD, Country::MDA);
        map.insert(Iso2::ME, Country::MNE);
        map.insert(Iso2::MF, Country::MAF);
        map.insert(Iso2::MG, Country::MDG);
        map.insert(Iso2::MH, Country::MHL);
        map.insert(Iso2::MI, Country::MID);
        map.insert(Iso2::MK, Country::MKD);
        map.insert(Iso2::ML, Country::MLI);
        map.insert(Iso2::MM, Country::MMR);
        map.insert(Iso2::MN, Country::MNG);
        map.insert(Iso2::MO, Country::MAC);
        map.insert(Iso2::MP, Country::MNP);
        map.insert(Iso2::MQ, Country::MTQ);
        map.insert(Iso2::MR, Country::MRT);
        map.insert(Iso2::MS, Country::MSR);
        map.insert(Iso2::MT, Country::MLT);
        map.insert(Iso2::MU, Country::MUS);
        map.insert(Iso2::MV, Country::MDV);
        map.insert(Iso2::MW, Country::MWI);
        map.insert(Iso2::MX, Country::MEX);
        map.insert(Iso2::MY, Country::MYS);
        map.insert(Iso2::MZ, Country::MOZ);

        map.insert(Iso2::NA, Country::NAM);
        map.insert(Iso2::NC, Country::NCL);
        map.insert(Iso2::NE, Country::NER);
        map.insert(Iso2::NF, Country::NFK);
        map.insert(Iso2::NG, Country::NGA);
        map.insert(Iso2::NI, Country::NIC);
        map.insert(Iso2::NL, Country::NLD);
        map.insert(Iso2::NO, Country::NOR);
        map.insert(Iso2::NP, Country::NPL);
        map.insert(Iso2::NR, Country::NRU);
        map.insert(Iso2::NU, Country::NIU);
        map.insert(Iso2::NZ, Country::NZL);

        map.insert(Iso2::OM, Country::OMN);

        map.insert(Iso2::PA, Country::PAN);
        map.insert(Iso2::PE, Country::PER);
        map.insert(Iso2::PF, Country::PYF);
        map.insert(Iso2::PG, Country::PNG);
        map.insert(Iso2::PH, Country::PHL);
        map.insert(Iso2::PK, Country::PAK);
        map.insert(Iso2::PL, Country::POL);
        map.insert(Iso2::PM, Country::SPM);
        map.insert(Iso2::PN, Country::PCN);
        map.insert(Iso2::PR, Country::PRI);
        map.insert(Iso2::PS, Country::PSE);
        map.insert(Iso2::PT, Country::PRT);
        map.insert(Iso2::PW, Country::PLW);
        map.insert(Iso2::PY, Country::PRY);

        map.insert(Iso2::QA, Country::QAT);

        map.insert(Iso2::RE, Country::REU);
        map.insert(Iso2::RO, Country::ROU);
        map.insert(Iso2::RS, Country::SRB);
        map.insert(Iso2::RU, Country::RUS);
        map.insert(Iso2::RW, Country::RWA);

        map.insert(Iso2::SA, Country::SAU);
        map.insert(Iso2::SB, Country::SLB);
        map.insert(Iso2::SC, Country::SYC);
        map.insert(Iso2::SD, Country::SDN);
        map.insert(Iso2::SE, Country::SWE);
        map.insert(Iso2::SG, Country::SGP);
        map.insert(Iso2::SH, Country::SHN);
        map.insert(Iso2::SI, Country::SVN);
        map.insert(Iso2::SJ, Country::SJM);
        map.insert(Iso2::SK, Country::SVK);
        map.insert(Iso2::SL, Country::SLE);
        map.insert(Iso2::SM, Country::SMR);
        map.insert(Iso2::SN, Country::SEN);
        map.insert(Iso2::SO, Country::SOM);
        map.insert(Iso2::SR, Country::SUR);
        map.insert(Iso2::SS, Country::SSD);
        map.insert(Iso2::ST, Country::STP);
        map.insert(Iso2::SV, Country::SLV);
        map.insert(Iso2::SX, Country::SXM);
        map.insert(Iso2::SY, Country::SYR);
        map.insert(Iso2::SZ, Country::SWZ);

        map.insert(Iso2::TC, Country::TCA);
        map.insert(Iso2::TD, Country::TCD);
        map.insert(Iso2::TF, Country::ATF);
        map.insert(Iso2::TG, Country::TGO);
        map.insert(Iso2::TH, Country::THA);
        map.insert(Iso2::TJ, Country::TJK);
        map.insert(Iso2::TK, Country::TKL);
        map.insert(Iso2::TL, Country::TLS);
        map.insert(Iso2::TM, Country::TKM);
        map.insert(Iso2::TN, Country::TUN);
        map.insert(Iso2::TO, Country::TON);
        map.insert(Iso2::TR, Country::TUR);
        map.insert(Iso2::TT, Country::TTO);
        map.insert(Iso2::TV, Country::TUV);
        map.insert(Iso2::TW, Country::TWN);
        map.insert(Iso2::TZ, Country::TZA);

        map.insert(Iso2::UA, Country::UKR);
        map.insert(Iso2::UG, Country::UGA);
        map.insert(Iso2::UM, Country::UMI);
        map.insert(Iso2::US, Country::USA);
        map.insert(Iso2::UY, Country::URY);
        map.insert(Iso2::UZ, Country::UZB);

        map.insert(Iso2::VA, Country::VAT);
        map.insert(Iso2::VC, Country::VCT);
        map.insert(Iso2::VE, Country::VEN);
        map.insert(Iso2::VG, Country::VGB);
        map.insert(Iso2::VI, Country::VIR);
        map.insert(Iso2::VN, Country::VNM);
        map.insert(Iso2::VU, Country::VUT);

        map.insert(Iso2::WF, Country::WLF);
        map.insert(Iso2::WS, Country::WSM);

        map.insert(Iso2::YE, Country::YEM);

        map.insert(Iso2::YT, Country::MYT);

        map.insert(Iso2::ZA, Country::ZAF);
        map.insert(Iso2::ZM, Country::ZMB);
        map.insert(Iso2::ZW, Country::ZWE);

        return map;
    };
}

pub fn find_country_by_iso(raw_iso: &str) -> Country {
    let country = match raw_iso.len() {
        2 => {
            let iso_code = Iso2::from_str(raw_iso);
            if iso_code.is_err() {
                panic!("{} is not a valid iso2 code", raw_iso)
            }
            let iso = iso_code.unwrap();
            match ISO2_MAP.get(&iso) {
                None => {
                    panic!("Country not found for {}", raw_iso)
                }
                Some(country) => country,
            }
        }
        // TODO: this code is broken
        3 => {
            let iso_code = Iso3::from_str(raw_iso);
            if iso_code.is_err() {
                panic!("{} is not a valid iso3 code", raw_iso)
            }
            let iso = Iso2::covert(iso_code.unwrap());
            match ISO2_MAP.get(&iso) {
                None => {
                    panic!("Country not found for {}", raw_iso)
                }
                Some(country) => country,
            }
        }
        _ => {
            panic!(
                "{} is not a valid iso code. Iso code needs to be 2 or 3 characters long, not {}",
                raw_iso,
                raw_iso.len()
            )
        }
    };
    return Country {
        name: country.name,
        iso2: country.iso2.clone(),
        iso3: country.iso3.clone(),
    };
}
