#[cfg(test)]
mod tests {
    use gowiz_country::country::NUMBER_OF_COUNTRIES;
    use gowiz_country::iso::{Iso2, Iso3};
    use gowiz_country::regions;
    use std::borrow::Borrow;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn every_country_has_unique_name() {
        let names: Vec<String> = regions::ALL_COUNTRIES
            .iter()
            .map(|country| country.name.to_string())
            .collect::<Vec<String>>();
        let unique_names: HashSet<&String> = HashSet::from_iter(names.iter());
        assert_eq!(NUMBER_OF_COUNTRIES as usize, unique_names.len());
    }

    #[test]
    fn every_country_has_unique_iso2() {
        let iso2: Vec<&Iso2> = regions::ALL_COUNTRIES
            .iter()
            .map(|country| country.iso2.borrow())
            .collect::<Vec<&Iso2>>();
        let unique_iso2: HashSet<&&Iso2> = HashSet::from_iter(iso2.iter());
        assert_eq!(NUMBER_OF_COUNTRIES as usize, unique_iso2.len());
    }
    #[test]
    fn every_country_has_unique_iso3() {
        let iso3: Vec<&Iso3> = regions::ALL_COUNTRIES
            .iter()
            .map(|country| country.iso3.borrow())
            .collect::<Vec<&Iso3>>();
        let unique_iso3: HashSet<&&Iso3> = HashSet::from_iter(iso3.iter());
        assert_eq!(NUMBER_OF_COUNTRIES as usize, unique_iso3.len());
    }

    #[test]
    fn every_country_has_no_empty_name() {
        let empty_countries: Vec<String> = regions::ALL_COUNTRIES
            .iter()
            .map(|country| country.name.to_string())
            .filter(|country| country.len() < 1)
            .collect::<Vec<String>>();
        let expected_result: Vec<String> = vec![];
        assert_eq!(expected_result, empty_countries);
    }
}
