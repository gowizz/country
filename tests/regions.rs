#[cfg(test)]
mod tests {
    use gowiz_country::country::NUMBER_OF_COUNTRIES;
    use gowiz_country::regions;

    #[test]
    fn every_country_is_present() {
        assert_eq!(NUMBER_OF_COUNTRIES as usize, regions::ALL_COUNTRIES.len());
    }
}
