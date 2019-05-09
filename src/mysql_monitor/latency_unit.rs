use yaml_rust::Yaml;

#[derive(Debug)]
pub enum LatencyUnit {
    Minutes,
    Seconds,
    Milliseconds,
    Nanoseconds,
}

impl LatencyUnit {
    fn convert_to_millis(&self, value: u16) -> u16 {
        match *self {
            LatencyUnit::Minutes => value * 60 * 1000,
            LatencyUnit::Seconds => value * 1000,
            LatencyUnit::Milliseconds => value,
            LatencyUnit::Nanoseconds => value / 1000,
        }
    }
}

pub fn parse_latency(doc: &Yaml) -> LatencyUnit {
    match doc["cachet"]["latency_unit"].as_str().as_ref() {
        Some(&"m") => LatencyUnit::Minutes,
        Some(&"s") => LatencyUnit::Seconds,
        Some(&"ms") => LatencyUnit::Milliseconds,
        Some(&"ns") => LatencyUnit::Nanoseconds,
        Some(&_) => LatencyUnit::Milliseconds,
        None => LatencyUnit::Milliseconds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_minutes_convert_to_millis() {
        assert_eq!(LatencyUnit::Minutes.convert_to_millis(1), 60000);
    }

    #[test]
    fn test_seconds_convert_to_millis() {
        assert_eq!(LatencyUnit::Seconds.convert_to_millis(1), 1000);
    }

    #[test]
    fn test_millis_convert_to_millis() {
        assert_eq!(LatencyUnit::Milliseconds.convert_to_millis(1), 1);
    }

    #[test]
    fn test_nanos_convert_to_millis() {
        assert_eq!(LatencyUnit::Nanoseconds.convert_to_millis(1000), 1);
    }

    #[test]
    fn test_parse_latency_minutes() {
        assert_latency(
            indoc!(
                "
cachet:
  latency_unit: m
        "
            ),
            LatencyUnit::Minutes,
        );
    }

    #[test]
    fn test_parse_latency_seconds() {
        assert_latency(
            indoc!(
                "
cachet:
  latency_unit: s
        "
            ),
            LatencyUnit::Seconds,
        );
    }

    #[test]
    fn test_parse_latency_millis() {
        assert_latency(
            indoc!(
                "
cachet:
  latency_unit: ms
        "
            ),
            LatencyUnit::Milliseconds,
        );
    }

    #[test]
    fn test_parse_latency_nanoseconds() {
        assert_latency(
            indoc!(
                "
cachet:
  latency_unit: ns
        "
            ),
            LatencyUnit::Nanoseconds,
        );
    }

    #[test]
    fn test_parse_latency_invalid_unit() {
        assert_latency(
            indoc!(
                "
cachet:
  latency_unit: wrong
        "
            ),
            LatencyUnit::Milliseconds,
        );
    }

    /// Will parse the given str reference (yaml string) and verify it matches the given expected_latency.
    ///
    /// # Arguments
    /// * `yaml_string` - The yaml string that should represent a cachethq configuration, including the latency_unit.
    /// * `expected_latency` - The expected LatencyUnit enum, extracted from the yaml string.
    fn assert_latency(yaml_string: &str, expected_latency: LatencyUnit) {
        let docs = YamlLoader::load_from_str(yaml_string).unwrap();
        let parsed_latency = parse_latency(&docs[0]);
        if let expected_latency = parsed_latency {
            println!("Success");
        } else {
            panic!(
                "parse_latency() should return {:?}. Got: {:?}",
                &expected_latency, &parsed_latency
            );
        }
    }
}
