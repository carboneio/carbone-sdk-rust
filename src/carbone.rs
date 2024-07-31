use bytes::Bytes;

use std::path::Path;
use std::time::Duration;

use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::multipart;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::StatusCode;
use reqwest::Response;


use crate::carbone_response::APIResponse;
use crate::config::Config;
use crate::errors::*;
use crate::render::*;
use crate::template::*;
use crate::types::{ApiJsonToken, JsonData};

use crate::types::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Carbone<'a> {
    config: &'a Config,
    http_client: Client,
}

impl<'a> Carbone<'a> {
    pub fn new(config: &'a Config, api_token: Option<&'a ApiJsonToken>) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "carbone-version",
            HeaderValue::from_str(config.api_version.as_str()).unwrap(),
        );

        let bearer = format!("Bearer {}", api_token.expect("REASON").as_str());

        let mut auth_value = header::HeaderValue::from_str(bearer.as_str()).unwrap();
        auth_value.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth_value);

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.api_timeout))
            .build()?;

        Ok(Self {
            config,
            http_client,
        })
    }

    // Delete a template from the Carbone Service.
    pub async fn delete_template(&self, template_id: TemplateId) -> Result<bool> {
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.delete(url).send().await?;

        let json = response.json::<APIResponse>().await?;

        if json.success {
            Ok(true)
        } else {
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }

    // Download a template from the Carbone Service.
    pub async fn download_template(&self, template_id: &TemplateId) -> Result<Bytes> {
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.get(url).send().await?;

        if response.status() == StatusCode::OK {
            Ok(response.bytes().await?)
        } else {
            let json = response.json::<APIResponse>().await?;
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }

    /// Generate a report.
    pub async fn generate_report(
        &self,
        template_name: String,
        template_data: Vec<u8>,
        json_data: JsonData,
        payload: Option<&str>,
        salt: Option<&str>
    ) -> Result<Bytes> {

        let template_id_generated = TemplateId::from_bytes(template_data.to_owned(), payload)?;
        let mut template_id = template_id_generated;
        let mut render_id = None;
    
        match self.render_data(template_id, json_data.clone()).await {
            Ok(id) => {
                render_id = Some(id);
            }
            Err(e) => match e {
                CarboneError::HttpError { status_code, error_message } => {
                    if status_code == reqwest::StatusCode::NOT_FOUND {
                        template_id = self.upload_template(template_name.as_str(), template_data, salt).await?;
                        render_id = Some(self.render_data(template_id, json_data).await?);
                    } else {
                        return Err(CarboneError::HttpError { status_code, error_message });
                    }
                },
                CarboneError::Error(error_message) => {
                    return Err(CarboneError::Error(error_message));
                },
                _ => {
                    return Err(e);
                }
            }
        };
    
        let report_content = self.get_report(&render_id.unwrap()).await?;
    
        Ok(report_content)
    }


    /// Get a new report.
    pub async fn get_report(&self, render_id: &RenderId) -> Result<Bytes> {
        let url = format!("{}/render/{}", self.config.api_url, render_id.as_str());

        let response = self.http_client.get(url).send().await?;

        // let mut report_name = None;

        // if let Some(content_disposition) = response.headers().get("content-disposition") {
        //     if let Ok(disposition) = content_disposition.to_str() {
        //         let split_content_disposition: Vec<&str> = disposition.split('=').collect();

        //         if split_content_disposition.len() == 2 {
        //             let mut name = split_content_disposition[1].to_string();
        //             if name.starts_with('"') && name.ends_with('"') {
        //                 name = name[1..name.len() - 1].to_string();
        //             }
        //             report_name = Some(name);
        //         }
        //     }
        // }

        if response.status() == StatusCode::OK {
            Ok(response.bytes().await?)
        } else {
            let json = response.json::<APIResponse>().await?;
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }

    /// Generate a report with a template_id given.
    pub async fn generate_report_with_template_id(
        &self,
        template_id: TemplateId,
        json_data: JsonData,
    ) -> Result<Bytes> {
        let render_id = self.render_data(template_id, json_data).await?;
        let report_content = self.get_report(&render_id).await?;

        Ok(report_content)
    }

    /// Render data with a given template_id.
    pub async fn render_data(
        &self,
        template_id: TemplateId,
        json_data: JsonData,
    ) -> Result<RenderId> {
        let url = format!("{}/render/{}", self.config.api_url, template_id.as_str());

        let response = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_data.as_str().to_owned())
            .send()
            .await?;

        if !response.status().is_success() {
            let status_code = response.status();
            let json = response.json::<APIResponse>().await?;
            return Err(CarboneError::HttpError {
                status_code,
                error_message: json.error.unwrap_or_else(|| "Unknown error".to_string()),
            });
        }

        let json = response.json::<APIResponse>().await?;

        if json.success {
            Ok(json.data.unwrap().render_id.unwrap())
        } else {
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }

    /// Upload a template to the Carbone Service.
    pub async fn upload_template(
        &self,
        file_name: &str,
        file_content: Vec<u8>,
        salt: Option<&str>,
    ) -> Result<TemplateId> {
        let salt = match salt {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        let file_path = Path::new(file_name);

        let file_name = file_path
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned());

        let file_name = match file_name {
            Some(s) => s,
            None => return Err(CarboneError::Error("Failed to fetch file name".to_string())),
        };


        let ext = file_path
            .extension()
            .and_then(|ext| ext.to_str()) .unwrap_or("");
        let mime = mime_guess::from_ext(ext).first_or_octet_stream();

        let part = multipart::Part::bytes(file_content)
            .file_name(file_name)
            .mime_str(mime.as_ref())?;

        let form: multipart::Form = multipart::Form::new().text("", salt).part("template", part);

        let url = format!("{}/template", self.config.api_url);


        let response = self.http_client.post(url).multipart(form).send().await?;

        let json = response.json::<APIResponse>().await?;

        if json.success {
            Ok(json.data.unwrap().template_id.unwrap())
        } else {
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }


    pub async fn get_status(&self) -> Result<String>
    {
        let url = format!("{}/status", self.config.api_url);

        let response = self.http_client.get(url).send().await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            let json = response.json::<APIResponse>().await?;
            Err(CarboneError::Error(json.error.unwrap()))
        }
    }
}
