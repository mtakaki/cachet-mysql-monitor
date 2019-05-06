use core::fmt::Debug;

pub fn update_status(cachet_hq: super::mysql_monitor::CachetHQ, status: u8) -> bool {
    let client = reqwest::Client::new();
    let params = [("id", cachet_hq.component_id), ("status", status as u16)];
    let result = client
        .put(&format!(
            "{}/components/{}",
            &cachet_hq.api_url, &cachet_hq.component_id
        ))
        .header("X-Cachet-Token", cachet_hq.token)
        .form(&params)
        .send();
    result.unwrap().status().is_success()
}
