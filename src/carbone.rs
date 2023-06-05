
use crate::carbone_response::CarboneSDKResponse;
use crate::config::Config;
use crate::errors::*;
use bytes::Bytes;
use reqwest::blocking::multipart;
use reqwest::header::HeaderValue;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::str;
use validator::Validate;

pub const CARBONE_API_URL: &str = "https://api.carbone.io";

pub type Result<T> = std::result::Result<T, CarboneSdkError>;


#[derive(Debug, Validate, PartialEq, Eq)]
pub struct CarboneSDK<'a>{
    pub config: &'a Config,
    #[validate(length(min = 300))]
    api_token: String,
}

impl <'a>CarboneSDK<'a> {
    pub fn new(config: &'a Config, api_token: String) -> Result<Self> {
        Ok(Self { config: config, api_token: api_token })
    }

    pub fn add_template(
        &self,
        template_file_name: &String,
        salt: String,
    ) -> Result<String> {
        if template_file_name.is_empty() {
            return Err(CarboneSdkError::MissingTemplateFileName);
        }

        if Path::new(template_file_name.as_str()).is_dir() {
            return Err(CarboneSdkError::IsADirectory(template_file_name.to_string()));
        }

        if !Path::new(template_file_name.as_str()).is_file() {
            return Err(CarboneSdkError::FileNotFound(template_file_name.to_string()));
        }

        let form = multipart::Form::new()
            .text("", salt)
            .file("template", template_file_name)?;

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template", self.config.api_url);

        // TODO move new client to new() method
        let response = client
            .post(url)
            .multipart(form)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<CarboneSDKResponse>()?;
                let template_id = json.get_template_id();
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(template_id)
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }

    pub fn get_template(&self, template_id: &String) -> Result<Bytes> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => Ok(response.bytes()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }

    pub fn delete_template(&self, template_id: &String) -> Result<()> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id);

        // TODO move new client to new() method
        let response = client
            .delete(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<CarboneSDKResponse>()?;
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(())
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }

    pub fn render_report(
        &self,
        template_id: &String,
        render_options: String,
    ) -> Result<String> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }
        if render_options.is_empty() {
            return Err(CarboneSdkError::MissingRenderOptions);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, template_id);

        // TODO move new client to new() method
        let response = client
            .post(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_token)
            .body(render_options)
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<CarboneSDKResponse>()?;
                let render_id = json.get_render_id();
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(render_id)
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }

    // TODO return also name of the report from headers
    pub fn get_report(&self, render_id: &String) -> Result<Bytes> {
        if render_id.is_empty() {
            return Err(CarboneSdkError::MissingRenderId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, render_id);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => Ok(response.bytes()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }

    // TODO: add payload to disgest
    pub fn generate_template_id(
        &self,
        template_file_name: &String,
        payload: &str,
    ) -> Result<String> {
        if template_file_name.is_empty() {
            return Err(CarboneSdkError::MissingTemplateFileName);
        }

        let file_content = fs::read(template_file_name)?;

        let mut sha256 = Sha256::new();

        sha256.update(payload);
        sha256.update(file_content);

        // convert [u8] to String
        let result: String = format!("{:X}", sha256.finalize());

        Ok(result.to_lowercase())
    }

    pub fn render(
        &self,
        file_or_template_id: &str,
        json_data: &str,
        payload: &str,
    ) -> Result<()> {
       panic!("function not implemented");
    }

    pub fn get_report_name_from_header(&self) -> String {
        "get_report_name_from_header() to be implemented".to_string()
    }

    pub fn get_status(&self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/status", self.config.api_url);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => Ok(response.text()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }

}