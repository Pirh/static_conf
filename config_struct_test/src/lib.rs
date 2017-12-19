#![cfg(test)]

#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde_yaml;
extern crate toml;


mod config;


mod ron_tests
{
    use ron;

    use config::ron::{ Config, CONFIG };

    #[test]
    fn test_declarations()
    {
        let _conf: &Config = &CONFIG;
    }

    #[test]
    fn test_deserialization()
    {
        let ron_source = include_str!("../config.ron");
        let conf: Config = ron::de::from_str(ron_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_simple_values()
    {
        assert_eq!(CONFIG.name, "Config name");
        assert_eq!(CONFIG.unit, ());
        assert_eq!(CONFIG.angelface, 'A');
        assert_eq!(CONFIG.integer, 100);
        assert_eq!(CONFIG.float, 100.1);
        assert_eq!(CONFIG.is_true, true);
        assert_eq!(CONFIG.nothing, None);
        assert_eq!(CONFIG.something, Some(10));
    }

    #[test]
    fn test_empty_array()
    {
        let empty: &[()] = &[];
        assert_eq!(CONFIG.empty, empty);
    }

    #[test]
    fn test_compound_values()
    {
        assert_eq!(CONFIG.countdown[0], 3);
        assert_eq!(CONFIG.countdown[1], 2);
        assert_eq!(CONFIG.countdown[2], 1);
        assert_eq!(CONFIG.structure.name, "Doesn't have one, sadly.");
        assert_eq!(CONFIG.structure.status, "Naw too bad.");
        assert_eq!(CONFIG.objects[0].name, "Thing 1");
        assert_eq!(CONFIG.objects[0].index, 0);
        assert_eq!(CONFIG.objects[1].name, "Thing 2");
        assert_eq!(CONFIG.objects[1].index, 1);
    }
}


mod toml_tests
{
    use toml;

    use config::toml::{ Config, CONFIG };

    #[test]
    fn test_declarations()
    {
        let _conf: &Config = &CONFIG;
    }

    #[test]
    fn test_deserialization()
    {
        let toml_source = include_str!("../config.toml");
        let conf: Config = toml::from_str(toml_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_simple_values()
    {
        assert_eq!(CONFIG.name, "Config name");
        assert_eq!(CONFIG.number, 100);
        assert_eq!(CONFIG.is_config, true);
        assert_eq!(CONFIG.is_not_config, false);
        assert_eq!(CONFIG.one_point_zero, 1.0);
        assert_eq!(CONFIG.one_point_five, 1.5);
        assert_eq!(CONFIG.floaty, 123.456789);
    }

    #[test]
    fn test_simple_array_values()
    {
        assert_eq!(CONFIG.coord, [-5.0, 5.0].as_ref());
        assert_eq!(CONFIG.color, [0, 64, 128, 255].as_ref());
        assert_eq!(CONFIG.words, ["one", "two", "three"].as_ref());
        assert_eq!(CONFIG.points, [[1, 2].as_ref(), [3, 4].as_ref(), [5, 6].as_ref()].as_ref());
    }

    #[test]
    fn test_table_values()
    {
        assert_eq!(CONFIG.table.name, "A table");
        assert_eq!(CONFIG.table.magnitude, 1000000000);
    }

    #[test]
    fn test_nested_tables()
    {
        assert_eq!(CONFIG.table.table_again.name, "OK this is just getting ridiculous");
        assert_eq!(CONFIG.table.table_again.description, "getting ridiculous");
    }

    #[test]
    fn test_array_of_tables()
    {
        assert_eq!(CONFIG.arrayble[0].description, "just unbelievable");
        assert_eq!(CONFIG.arrayble[1].description, "what is this syntax");
    }

    #[test]
    fn test_empty_array_is_array_of_unit()
    {
        let empty: &[()] = &[];
        assert_eq!(CONFIG.empty, empty);
    }
}


mod yaml_tests
{
    use std;
    use serde_yaml;

    use config::yaml::{ Config, CONFIG };

    #[test]
    fn test_declarations()
    {
        let _conf: &Config = &CONFIG;
    }

    #[test]
    fn test_deserialization()
    {
        let yaml_source = include_str!("../config.yaml");
        let conf: Config = serde_yaml::from_str(yaml_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_simple_values()
    {
        assert_eq!(CONFIG.name, "Config name");
        assert_eq!(CONFIG.nothing, None);
        assert_eq!(CONFIG.number, 100);
        assert_eq!(CONFIG.is_config, true);
        assert_eq!(CONFIG.is_not_config, false);
        assert_eq!(CONFIG.i64_max, std::i64::MAX);
        assert_eq!(CONFIG.u64_max, std::u64::MAX);
        assert_eq!(CONFIG.floaty, 123.456789);
    }

    #[test]
    fn test_composite_values()
    {
        assert_eq!(CONFIG.coord, [-5.0, 5.0].as_ref());
        assert_eq!(CONFIG.nested.name, "nested2");
        assert_eq!(CONFIG.nested.values.x, 0);
        assert_eq!(CONFIG.nested.values.y, 1);
        assert_eq!(CONFIG.nested.values.z, 2);
        assert_eq!(CONFIG.array_of_structs[0].name, "first");
        assert_eq!(CONFIG.array_of_structs[1].name, "second");
        assert_eq!(CONFIG.array_of_structs[0].n, 0);
        assert_eq!(CONFIG.array_of_structs[1].n, 1);
    }

    #[test]
    fn test_empty_array_is_array_of_unit()
    {
        let empty: &[()] = &[];
        assert_eq!(CONFIG.empty, empty);
    }
}

