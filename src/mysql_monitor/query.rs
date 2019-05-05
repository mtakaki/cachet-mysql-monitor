use mysql::Row;
use std::time::Instant;

pub fn query(mysql_monitor: &super::mysql_monitor::MySQLMonitor) -> (Vec<Row>, u128) {
    // One improvement could be keeping the connection pool open, but it depends on how sparse are
    // the queries.
    let pool = mysql::Pool::new(&mysql_monitor.mysql_uri).unwrap();

    let now = Instant::now();
    let rows: Vec<Row> = pool
        .prep_exec(&mysql_monitor.query, ())
        .map(|result| result.map(|x| x.unwrap()).map(|row| row).collect())
        .unwrap();

    return (rows, now.elapsed().as_millis());
}
