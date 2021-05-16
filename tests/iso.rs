#[cfg(test)]
mod tests {
    use gowiz_country::iso::find_country_by_iso;
    use gowiz_country::models::iso::{Iso2, Iso3};

    #[test]
    fn every_iso2_has_a_country() {
        for i in Iso2::iter() {
            let country = find_country_by_iso(&*i.to_string());
            assert_eq!(country.iso2, i.clone());
        }
    }

    #[test]
    fn every_iso3_has_a_country() {
        for i in Iso3::iter() {
            let country = find_country_by_iso(&*i.to_string());
            assert_eq!(country.iso3, i.clone());
        }
    }

    #[test]
    #[should_panic(
        expected = "x is not a valid iso code. Iso code needs to be 2 or 3 characters long, not 1"
    )]
    fn should_panic_for_random_text() {
        find_country_by_iso("x");
    }

    #[test]
    #[should_panic(expected = "xx is not a valid iso2 code")]
    fn should_panic_for_random_iso2() {
        find_country_by_iso("xx");
    }

    #[test]
    #[should_panic(expected = "xxx is not a valid iso3 code")]
    fn should_panic_for_random_iso3() {
        find_country_by_iso("xxx");
    }
}
