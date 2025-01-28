mod test {
    use jscontact::{Calendar, Resource};

    #[test]
    fn test_resource_to_calendar() {
        let resource = Resource::new("my_uri".to_string());
        assert_eq!(resource.uri, "my_uri");
        let calendar: Calendar = resource.into();
        assert_eq!(calendar.uri, "my_uri");

        let my_calendar = Calendar::new("my_uri");
        assert_eq!(my_calendar.uri, "my_uri");

        assert_eq!(calendar, my_calendar);
    }
}
