use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

/// Simple CLI: `httpinfo` or `httpinfo https://example.com`
fn main() -> Result<()> {
    let url = std::env::args()
            .nth(1)
                    .unwrap_or_else(|| "https://httpbin.org/get".into());

                        let client = Client::builder()
                                .timeout(Duration::from_secs(10))
                                        .gzip(true) // transparent decompression
                                                .brotli(true)
                                                        .build()
                                                                .context("build HTTP client")?;

                                                                    let mut headers = HeaderMap::new();
                                                                        headers.insert(
                                                                                USER_AGENT,
                                                                                        HeaderValue::from_static("httpinfo/0.1.0 (github.com/you/repo)"),
                                                                                            );

                                                                                                let resp = client
                                                                                                        .get(&url)
                                                                                                                .headers(headers)
                                                                                                                        .send()
                                                                                                                                .with_context(|| format!("GET {url}"))?;

                                                                                                                                    let status = resp.status();
                                                                                                                                        let hdrs = resp.headers().clone();
                                                                                                                                            let body = resp.text().context("read response body")?;

                                                                                                                                                // Pretty output
                                                                                                                                                    println!("Status : {status}");
                                                                                                                                                        println!("Headers:");
                                                                                                                                                            for (k, v) in hdrs {
                                                                                                                                                                    println!("  {}: {}", k.unwrap(), v.to_str().unwrap_or("<binary>"));
                                                                                                                                                                        }

                                                                                                                                                                            // Try to print JSON nicely, otherwise raw text
                                                                                                                                                                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                                                                                                                                                                                        println!("Body:\n{}", serde_json::to_string_pretty(&json)?);
                                                                                                                                                                                            } else {
                                                                                                                                                                                                    println!("Body:\n{body}");
                                                                                                                                                                                                        }

                                                                                                                                                                                                            Ok(())
                                                                                                                                                                                                            }
