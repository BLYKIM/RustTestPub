use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub fn to_timestamp_nano() {
    let str = "2023-02-01T00:16:18.207817703+00:00";

    let datetime = NaiveDateTime::parse_from_str(str, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap();
    let timestamp_nano = datetime.timestamp_nanos_opt().unwrap_or(i64::MAX);
    println!("{timestamp_nano}");

    println!("{}", Utc.timestamp_nanos(1_692_766_493_000_000_000));
}

// 1675210578199601467
// 1675210578207817703

// pub fn difference() {
//     let str1 = r#"curl -k 'https://172.30.1.212:8443/archive/graphql' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: https://172.30.1.212:8443' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJTeXN0ZW0gQWRtaW5pc3RyYXRvciIsImV4cCI6MTY4MTg5OTAwMX0.HKhT4dJiKAMu-xW_pJx30gPcCSYFRBdwu2MQYgmXlWs' --data-binary '{"query":"query http_data{\n httpRawEvents(filter:{\n  source:\"collect\"\n  time:{\n   start:\"2022-02-15T02:31:28.852876824+00:00\"\n   end:\"2024-02-15T02:31:28.852876824+00:00\"\n  }\n } ){\n  pageInfo{\n   hasNextPage\n   startCursor\n   endCursor\n  }\n   nodes{\n    timestamp\n    origAddr\n    origPort\n    respAddr\n    respPort\n    proto\n    lastTime\n    method\n    host\n    uri\n    referrer\n    version\n    userAgent\n    requestLen\n    responseLen\n    statusCode\n    statusMsg\n    username\n    password\n    cookie\n    contentEncoding\n    contentType\n    cacheControl\n   }\n }\n}\n"}' --compressed"#;
//     let str2 = r#"curl -k 'https://172.30.1.212:8443/archive/graphql' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: https://172.30.1.212:8443' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJTeXN0ZW0gQWRtaW5pc3RyYXRvciIsImV4cCI6MTY4MTg5OTAwMX0.HKhT4dJiKAMu-xW_pJx30gPcCSYFRBdwu2MQYgmXlWs' --data-binary '{"query":"query http_data{\n httpRawEvents(filter:{\n  source:\"collect\"\n  time:{\n   start:\"2022-02-15T02:31:28.852876824+00:00\"\n   end:\"2024-02-15T02:31:28.852876824+00:00\"\n  }\n } ){\n  pageInfo{\n   hasNextPage\n   startCursor\n   endCursor\n  }\n   nodes{\n    timestamp\n    origAddr\n    origPort\n    respAddr\n    respPort\n    proto\n    lastTime\n    method\n    host\n    uri\n    referrer\n    version\n    userAgent\n    requestLen\n    responseLen\n    statusCode\n    statusMsg\n    username\n    password\n    cookie\n    contentEncoding\n    contentType\n    cacheControl\n   }\n }\n}\n"}' --compressed"#;
//     let len = str1.len();
//     let chars1 = str1.char_indices();
//     for (i, ch) in chars1 {
//         if ch != str2[i] {
//             println!("gotcha {}", str1[i]);
//         }
//     }
// }
