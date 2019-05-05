use mysql::Row;
use std::time::Instant;

mod mysql_monitor;

fn main() {
    let (cachet_hq, mysql_monitor) = mysql_monitor::mysql_monitor::parse_config("config.yml");

    let (rows, elapsed) = mysql_monitor::query::query(&mysql_monitor);

    for expectation in mysql_monitor.expectations {
        if let mysql_monitor::expectation::Expectation::Rows(rows_number) = expectation {
            println!("Rows: {:?}; Expected: {:?}", &rows.len(), &rows_number);
            if rows.len() as u16 != rows_number {
                println!("Rows different from expected. Reporting incident...");
            }
        } else if let mysql_monitor::expectation::Expectation::Latency(millis) = expectation {
            println!("Query elapsed: {}ms; Expected: < {}ms", &elapsed, &millis);

            if (elapsed as u16) >= millis {
                println!(
                    "Query elapsed time is above or equals to the threshold. Reporting incident..."
                );
            }
        }
    }
}
