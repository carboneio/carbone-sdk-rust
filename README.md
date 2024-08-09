[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://github.com/carboneio/carbone-sdk-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/carboneio/carbone-sdk-rust/actions/workflows/rust.yml)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)



# Carbone-sdk-rust

Use the Carbone Rust SDK to communicate with the [Carbone API](https://carbone.io/api-reference.html) to generate documents. 

## Installation

```toml
[dependencies]
carbone-sdk-rust = "1.0.0"
```

## Quickstart

Try the following code to render a report in 10 seconds. Just insert your API key, the template ID you want to render, and the JSON data-set as string. Get your API key on your Carbone account: https://account.carbone.io/.

```rust
use std::env;
 
use carbone_sdk_rust::config::Config;
use carbone_sdk_rust::carbone::Carbone;
use carbone_sdk_rust::types::{ApiJsonToken, JsonData};
use carbone_sdk_rust::template::TemplateId;
 
use carbone_sdk_rust::errors::CarboneError;

use std::fs::File;
use std::io::Write

#[tokio::main]
async fn main() -> Result<(), CarboneError> {
    
    let token = "Token";

    let config: Config = Default::default();
 
    let api_token = ApiJsonToken::new(token.to_string())?;

    let json_data_value = String::from(r#"
        {
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        }
    "#);
 
    let json_data = JsonData::new(json_data_value)?;

    let template_id = TemplateId::new("YourTemplateId".to_string())?;

    let carbone = Carbone::new(&config, Some(&api_token))?;
    
    let report_content = match carbone.generate_report_with_template_id(template_id, json_data).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };

    let mut output_file = File::create("report.pdf").expect("Failed to create file");

    if let Err(e) = output_file.write_all(&report_content) {
        eprintln!("Failed to write to file: {:?}", e);
    }

    Ok(())
}
```

## Rust SDK API

### Table of content

- SDK functions:
    - [SDK Constructor](#sdk-constructor)
    - [Generate and Download a Document](#generate-and-download-document)
    - [Generate a Document Only](#generate-document-only)
    - [Download a Document Only](#download-document-only)
    - [Add a Template](#add-template)
    - [Delete a Template](#delete-template)
    - [Get a Template](#get-template)
    - [Get API status](#get-api-status)
    - [Set API Config](#set-api-config)
- [Build commands](#build-commands)
- [Test commands](#test-commands)
- [Contributing](#-contributing)

### SDK Constructor

Example of a new SDK instance for **Carbone Cloud**:
Get your API key on your Carbone account: https://account.carbone.io/.
```rust
// For Carbone Cloud, provide your API Access Token as first argument:
let token = "Token";
let config: Config = Default::default();
let api_token = ApiJsonToken::new(token.to_string())?;
let carbone = Carbone::new(&config, Some(&api_token))?;
```

Example of a new SDK instance for **Carbone On-premise** or **Carbone On-AWS**:
```rust
// Define the URL of your Carbone On-premise Server or AWS EC2 URL:
let config: Config = Config::new("ON_PREMISE_URL".to_string(), "api_time_out_in_sec_in_u64", ApiVersion::new("4".to_string()).expect("REASON")).expect("REASON");
let carbone = Carbone::new(&config, None)?;
```

### Generate and Download Document

Generate a document from a local template file:
```rust
pub async fn generate_report( &self, template_name: String, template_data: Vec<u8>, json_data: JsonData, payload: Option<&str>, salt: Option<&str>);
```

Arguments details:
* template_name: filename of the template.
* template_data: The content of the file in `Vec<u8>`.
* json_data: A stringified JSON containing the data to populate the template.


**Example**

```rust
let file_name = "name_file.extention";
let file_path = format!("your/path/{}", file_name);
let file_content = fs::read(file_path)?;

let json_data_value = String::from(r#"
        {
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        }
    "#);

let json_data = JsonData::new(json_data_value)?;

let content = match carbone.generate_report(file_name.to_string(), file_content, json_data, None, None).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };

```

**Or**, Generate a document from a template ID:
```rust
pub async fn pub async fn generate_report_with_template_id( &self, template_id: TemplateId, json_data: JsonData);
```
Argument details:
* template_id: Template ID (Manage your templates on [Carbone Studio](https://studio.carbone.io))
* json_data: A stringified JSON containing the data to populate the template.

```rust
let template_id = TemplateId::new("template_id".to_string())?;
let json_data = String::from(r#"
        {
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        }
    "#);

let json_data = JsonData::new(json_data_value)?;

let content = match carbone.generate_report_with_template_id( template_id, filte_content, json_data).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };
```

### Add Template

```rust
pub async fn upload_template(&self,file_name: &str,file_content: Vec<u8>,salt: Option<&str>);
```

Add a template as file-content `Vec<u8>` and the function return the template ID as `String`.

**Example**

```rust

let template_name = "template.odt".to_string();
let template_path = format!("src/{}", template_name);
let template_data = fs::read(template_path.to_owned())?;

let template_id = match carbone.upload_template(template_name, template_data, None).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };
```

### Delete Template

```rust
pub async fn delete_template(&self, template_id: TemplateId);
```

Delete a template by providing a template ID as `template_id`, and it returns whether the request succeeded as a `Boolean`.

**Example**

```rust
let template_id = TemplateId::new("template_id".to_string())?;

let boolean = match carbone.delete_template(template_id).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };
```

### Generate Document Only

The generate_report function takes a template ID as `String`, and the JSON data-set as `JsonData`.
It return a `renderId`, you can pass this `renderId` at [get_report](#download-document-only) for download the document.

```rust
pub async fn render_data( &self, template_id: TemplateId, json_data: JsonData);
```

**Example**

```rust

let template_id = TemplateId::new("template_id".to_string())?;

let json_data = String::from(r#"
        {
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        }
    "#);

let json_data = JsonData::new(json_data_value)?;

let render_id = match carbone.render_data(template_id, json_data).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };


```

### Download Document Only

**Definition**
```rust
pub async fn get_report(&self, render_id: &RenderId);
```

**Example**

```rust

let render_id = RenderId::new("render_id".to_string())?;

let content = match carbone.get_report(&render_id).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };

```

### Get Template

**Definition**

```rust
pub async fn download_template(&self, template_id: &TemplateId);
```

Provide a template ID as `String` and it returns the file as `Bytes`.

**Example**

```rust
let template_id = TemplateId::new("template_id".to_string())?;

let content = match carbone.download_template(&template_id).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };
```

### Get API Status

**Definition**

```rust

pub async fn get_status(&self);

```

The function requests the Carbone API to get the current status and version as `String`.

**Example**

```rust
let status = match carbone.get_status().await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };
```

### Set API Config

**Definition**

```rust
pub fn new(api_url: String, api_timeout: u64, api_version: ApiVersion)
```

Set the API URL for Carbone On-premise or Carbone On-AWS.

Specify the version of the Carbone CLoud API you want to request as second argument of the constructor.
By default, all requested are made to the Carbone API version `4`.

**Example**

```rust
let config: Config = Config::new("ON_PREMISE_URL".to_string(), "api_time_out_in_sec_in_u64", ApiVersion::new("Version".to_string()).expect("REASON")).expect("REASON");

let carbone = Carbone::new(&config, None)?;
```

## Build commands

At the root of the SDK repository run:
```sh
cargo build
```

In another Rust project, you can load the local build of the SDK, in the Cargo.toml:
```toml

carbone-sdk-rust = {path = "your/local/path"}
```
Finally, compile your Rust project with the SDK:
```sh
cargo run 
```

## Test commands

Execute unit tests:
```sh
cargo test
```
Execute unit tests with coverage:
```sh
cargo tarpaulin
```

## 👤 History

The package was originaly made by [Pascal Chenevas](https://github.com/pascal-chenevas), and open-sourced the code. The Carbone.io team is now maintaining the SDK and will bring all futur evolutions.

## 🤝 Contributing

Contributions, issues and feature requests are welcome!
Feel free to check [issues page](https://github.com/carboneio/carbone-rust-java/issues).

## Show your support

Give a ⭐️ if this project helped you!