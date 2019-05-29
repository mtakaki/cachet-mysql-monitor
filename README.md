# Status
[![CircleCI](https://circleci.com/gh/mtakaki/cachet-mysql-monitor.svg?style=svg)](https://circleci.com/gh/mtakaki/cachet-mysql-monitor)
![License](https://img.shields.io/github/license/mtakaki/cachet-mysql-monitor.svg)
[![](https://images.microbadger.com/badges/image/mtakaki/cachet-mysql-monitor.svg)](https://microbadger.com/images/mtakaki/cachet-mysql-monitor "Get your own image badge on microbadger.com")

cachet-mysql-monitor
========================
Rust plugin for [cachet](cachethq.io) that monitors a MySQL instance, verifying the number of rows returned by a query and the latency. The frequency the database is tested is configurable.

## Configuration

```yaml
mysql:
  uri: mysql://root:123@localhost:3306/test
  query: SELECT 1
  timeout: 0.01
  expectation:
    - type: ROWS
      value: 2
    - type: LATENCY
      threshold: 1000
  allowed_fails: 0
cachet:
  api_url: https://demo.cachethq.io/api/v1
  token: my_token
  component_id: 1
  metric_id: 1
  action:
    - CREATE_INCIDENT
    - UPDATE_STATUS
  public_incidents: true
  latency_unit: ms
frequency: 30
```
