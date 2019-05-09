use yaml_rust::Yaml;

#[derive(Debug, PartialEq)]
pub enum Expectation {
    Rows(u16),
    Latency(u16),
    Regex(String), // Still not properly implemented.
}

pub fn parse_expectations(doc: &Yaml) -> Vec<Expectation> {
    let mut expectations: Vec<Expectation> = Vec::new();

    for current_expectation in doc["mysql"]["expectation"].as_vec().unwrap() {
        match current_expectation["type"].as_str().as_ref() {
            Some(&"ROWS") => expectations.push(Expectation::Rows(
                current_expectation["value"].as_i64().unwrap() as u16,
            )),
            Some(&"LATENCY") => expectations.push(Expectation::Latency(
                current_expectation["threshold"].as_i64().unwrap() as u16,
            )),
            Some(&"REGEX") => expectations.push(Expectation::Regex(
                current_expectation["regex"].as_str().unwrap().to_owned(),
            )),
            Some(&_) => (),
            None => (),
        };
    }

    return expectations;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_parse_expectations_rows() {
        assert_expectation(
            indoc!(
                "
mysql:
  expectation:
    - type: ROWS
      value: 2
        "
            ),
            Some(Expectation::Rows(2)),
        );
    }

    #[test]
    fn test_parse_expectations_latency() {
        assert_expectation(
            indoc!(
                "
mysql:
  expectation:
    - type: LATENCY
      threshold: 10
        "
            ),
            Some(Expectation::Latency(10)),
        );
    }

    #[test]
    fn test_parse_expectations_regex() {
        assert_expectation(
            indoc!(
                "
mysql:
  expectation:
    - type: REGEX
      regex: \"foo.*\"
        "
            ),
            Some(Expectation::Regex("foo.*".to_owned())),
        );
    }

    #[test]
    fn test_parse_expectations_multiple() {
        let mut expected: Vec<Expectation> = Vec::new();
        expected.push(Expectation::Regex("foo.*".to_owned()));
        expected.push(Expectation::Latency(10));

        assert_expectations(
            indoc!(
                "
mysql:
  expectation:
    - type: REGEX
      regex: \"foo.*\"
    - type: LATENCY
      threshold: 10
        "
            ),
            expected,
        );
    }

    #[test]
    fn test_parse_expectations_empty() {
        assert_expectation(
            indoc!(
                "
mysql:
  expectation: []
        "
            ),
            None,
        );
    }

    #[test]
    fn test_parse_expectations_unknown() {
        assert_expectation(
            indoc!(
                "
mysql:
  expectation:
    - type: FOO
        "
            ),
            None,
        );
    }

    /// Will parse the given str reference (yaml string) and verify it matches the given expected_expectation.
    ///
    /// # Arguments
    /// * `yaml_string` - The yaml string that should represent the mysql configuration, including the expectation array.
    /// * `expected_expectation` - The expected Expectation enum, extracted from the yaml string.
    fn assert_expectation(yaml_string: &str, expected_expectation: Option<Expectation>) {
        let docs = YamlLoader::load_from_str(yaml_string).unwrap();
        let expectations = parse_expectations(&docs[0]);
        match expected_expectation {
            None => assert!(expectations.is_empty()),
            _ => assert!(expectations.contains(&expected_expectation.unwrap())),
        }
    }

    /// Will parse the given str reference (yaml string) and verify if each one of the expectations in the vector
    /// are contained inside the parsed expectations.
    ///
    /// # Arguments
    /// * `yaml_string` - The yaml string that should represent the mysql configuration, including the expectation array.
    /// * `expected_expectation_vector` - The expected Expectation enum vector, extracted from the yaml string.
    fn assert_expectations(yaml_string: &str, expected_expectation_vector: Vec<Expectation>) {
        let docs = YamlLoader::load_from_str(yaml_string).unwrap();
        let expectations = parse_expectations(&docs[0]);
        for expected_expectation in expected_expectation_vector {
            assert!(expectations.contains(&expected_expectation));
        }
    }
}
