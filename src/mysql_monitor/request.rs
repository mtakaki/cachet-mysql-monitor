#[cfg(test)]
use mockito;

/// Updates the component status in CachetHQ and returns the result, if it was successful or not.
///
/// # Arguments
///
/// * `cachet_hq` - The `CachetHQ` object from where we'll take the component id and token.
/// * `status` - The component status per https://docs.cachethq.io/docs/component-statuses
pub fn update_status(cachet_hq: super::mysql_monitor::CachetHQ, status: u8) -> bool {
    let client = reqwest::Client::new();
    let params = [("id", cachet_hq.component_id), ("status", status as u16)];
    let result = client
        .put(&format!(
            "{}/components/{}",
            &cachet_hq.api_url, &cachet_hq.component_id
        ))
        .header("X-Cachet-Token", cachet_hq.token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send();
    result.unwrap().status().is_success()
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::mysql_monitor::latency_unit::LatencyUnit;
    use crate::mysql_monitor::mysql_monitor::CachetHQ;

    use super::*;

    #[test]
    fn test_update_status() {
        let url = &mockito::server_url();

        // Mocking the HTTP request.
        let _m = mock("PUT", "/components/1")
            .with_status(201)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_header("X-Cachet-Token", "1234")
            .with_body("id=1&status=0")
            .create();

        let cachet_hq = CachetHQ {
            component_id: 1,
            token: "1234".to_string(),
            api_url: url.to_string(),
            latency_unit: LatencyUnit::Milliseconds,
            public_incidents: false,
            action: Vec::new(),
            metric_id: Some(3),
        };
        assert!(update_status(cachet_hq, 0));
    }
}
