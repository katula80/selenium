// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use assert_cmd::Command;
use rstest::rstest;
use std::str;

#[rstest]
#[case("chrome", "", "")]
#[case("chrome", "105", "105.0.5195.52")]
#[case("chrome", "106", "106.0.5249.61")]
#[case("edge", "", "")]
#[case("edge", "105", "105.0")]
#[case("edge", "106", "106.0")]
#[case("firefox", "", "")]
#[case("firefox", "105", "0.32.0")]
fn ok_test(
    #[case] browser: String,
    #[case] browser_version: String,
    #[case] driver_version: String,
) {
    println!(
        "CLI test browser={} -- browser_version={} -- driver_version={}",
        browser, browser_version, driver_version
    );

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", &browser, "--browser-version", &browser_version])
        .assert()
        .success()
        .code(0);

    let stdout = &cmd.unwrap().stdout;
    let output = match str::from_utf8(stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", output);

    assert!(output.contains("driver"));
    if !browser_version.is_empty() {
        assert!(output.contains(&driver_version));
    }
}

#[rstest]
#[case("wrong-browser", "", "", 1)]
#[case("chrome", "wrong-version", "", 1)]
#[case("chrome", "", "wrong-version", 101)]
fn error_test(
    #[case] browser: String,
    #[case] browser_version: String,
    #[case] driver_version: String,
    #[case] error_code: i32,
) {
    println!(
        "CLI test browser={} -- browser_version={} -- driver_version={}",
        browser, browser_version, driver_version
    );

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([
        "--browser",
        &browser,
        "--browser-version",
        &browser_version,
        "--driver-version",
        &driver_version,
    ])
    .assert()
    .failure()
    .code(error_code);
}
