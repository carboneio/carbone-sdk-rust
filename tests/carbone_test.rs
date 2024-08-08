use std::fs;

use httpmock::prelude::*;
use serde_json::json;

use carbone_sdk_rust::carbone::Carbone;
use carbone_sdk_rust::carbone_response::APIResponse;
use carbone_sdk_rust::errors::CarboneError;
use carbone_sdk_rust::render::*;
use carbone_sdk_rust::types::JsonData;
use reqwest::StatusCode;
mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use carbone_sdk_rust::{
        carbone_response::APIResponseData, config::Config, template::*, types::ApiVersion,
    };

    use super::*;

    #[tokio::test]
    async fn test_delete_template() -> Result<(), CarboneError> {
        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let body = APIResponse {
            success: true,
            data: None,
            error: None,
            code: None,
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200).json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let is_deleted = carbone.delete_template(template_id).await.unwrap();

        mock_server.assert();

        assert_eq!(is_deleted, true);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_template_failed() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let api_version = ApiVersion::new("4".to_string())?;
        let config = Config::new("http://bad_url".to_string(), 1, api_version)?;
        let api_token = helper.create_api_token()?;

        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let result = carbone.delete_template(template_id).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_template_unknown_template_id_given() -> Result<(), CarboneError> {
        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = APIResponse {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: None,
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let result = carbone.delete_template(template_id).await;

        let expected_error = CarboneError::Error(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_download() -> Result<(), CarboneError> {
        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let template_file_content = fs::read("tests/data/template.test.odt")?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200).body(template_file_content.clone());
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let template_content = carbone.download_template(&template_id).await.unwrap();

        mock_server.assert();

        assert_eq!(template_file_content, template_content.to_vec());

        Ok(())
    }

    #[tokio::test]
    async fn test_download_failed() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let api_version = ApiVersion::new("4".to_string())?;
        let config = Config::new("http://bad_url".to_string(), 1, api_version)?;
        let api_token = helper.create_api_token()?;

        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let result = carbone.download_template(&template_id).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_downaload_unknown_template_id_given() -> Result<(), CarboneError> {
        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = APIResponse {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: Some("w115".to_string()),
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let result = carbone.download_template(&template_id).await;

        let expected_error = CarboneError::Error(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_report_with_template_id() -> Result<(), CarboneError> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let report_data = fs::read_to_string("tests/data/report_data.json")?;

        let template_file = TemplateFile::new("tests/data/template.odt".to_string(), None)?;
        let template_id = template_file.generate_id(None)?;

        let json_data = JsonData::new(report_data)?;

        let render_id_value = "MTAuMjAuMjEuNDAgICAgBY4OM11wQg11ekv6_R0n0wcmVwb3J0.pdf".to_string();
        let _render_id = &RenderId::new(&render_id_value)?;

        let file_path = "tests/data/report.pdf";

        let expected_content = fs::read(file_path)?;

        let mock_render_response = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "renderId": &render_id_value,
                    "inputFileExtension": "odt"
                }
            }));
        });

        let mock_get_report_response = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", &render_id_value));
            then.status(200).body(&expected_content);
        });

        let result = carbone
            .generate_report_with_template_id(template_id, json_data)
            .await
            .unwrap();

        mock_render_response.assert();
        mock_get_report_response.assert();

        assert_eq!(result, expected_content);

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_report() -> Result<(), CarboneError> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let report_data = fs::read_to_string("tests/data/report_data.json")?;

        let template_name = "template.odt".to_string();
        let template_path = format!("tests/data/{}", template_name);
        let template_data = fs::read(template_path.to_owned())?;

        let template_file = TemplateFile::new(template_path, Some(template_data.to_owned()))?;
        let template_id = template_file.generate_id(None)?;

        let json_data = JsonData::new(report_data)?;

        let render_id_value = "MTAuMjAuMjEuNDAgICAgBY4OM11wQg11ekv6_R0n0wcmVwb3J0.pdf".to_string();
        let render_id = &RenderId::new(&render_id_value)?;

        let file_path = "tests/data/report.pdf";

        let expected_content = fs::read(file_path)?;

        let mock_render_response = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "renderId": render_id.as_str(),
                    "inputFileExtension": "odt"
                }
            }));
        });

        let mock_get_report_response = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(200).body(&expected_content);
        });

        let result = carbone
            .generate_report(template_name, template_data, json_data, None, None)
            .await
            .unwrap();

        mock_render_response.assert();
        mock_get_report_response.assert();

        assert_eq!(result, expected_content);

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_report_unupload_template() -> Result<(), CarboneError>
    {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let report_data = fs::read_to_string("tests/data/report_data.json")?;

        let template_name = "template2.odt".to_string();
        let template_path = format!("tests/data/{}", template_name);
        let template_data = fs::read(template_path.to_owned())?;

        let template_file = TemplateFile::new(template_path, Some(template_data.to_owned()))?;
        let template_id = template_file.generate_id(None)?;

        let template_id_expected = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let data = APIResponseData {
            template_id: Some(template_id_expected.clone()),
            render_id: None,
            template_file_extension: None,
        };

        let body = APIResponse {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        };

        let json_data = JsonData::new(report_data)?;

        let render_id_value = "MTAuMjAuMjEuNDAgICAgBY4OM11wQg11ekv6_R0n0wcmVwb3J0.pdf".to_string();
        let render_id = &RenderId::new(&render_id_value)?;

        let file_path = "tests/data/report.pdf";

        let expected_content = fs::read(file_path)?;

        let mock_render_response_false = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(404)
            .json_body(json!({
                "success": false,
                "error": "template no found"
            }));
        });

        let mock_upload_template = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/template"));
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let mock_render_response = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id_expected.as_str()));
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "renderId": render_id.as_str(),
                    "inputFileExtension": "odt"
                }
            }));
        });

        let mock_get_report_response = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(200).body(&expected_content);
        });

        let result = carbone
            .generate_report(template_name, template_data, json_data, None, None)
            .await
            .unwrap();

        mock_render_response.assert();
        mock_get_report_response.assert();

        assert_eq!(result, expected_content);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_report() -> Result<(), CarboneError> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let render_id_value =
            "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let rendered_file_content = fs::read("tests/data/report.pdf")?;

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(200).body(rendered_file_content.clone());
        });

        let report_content = carbone.get_report(render_id).await.unwrap();

        mock_server.assert();
        assert_eq!(report_content, rendered_file_content.to_vec());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_report_failed() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let api_version: ApiVersion = ApiVersion::new("4".to_string())?;
        let config = Config::new("http://bad_url".to_string(), 1, api_version)?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let render_id_value =
            "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let result = carbone.get_report(render_id).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_report_unknown_render_id_given() -> Result<(), CarboneError> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let render_id_value = "unknown_render_id.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = APIResponse {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: Some("w115".to_string()),
        };

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let expected_error = CarboneError::Error(error_msg);
        let result = carbone.get_report(render_id).await;

        mock_server.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_render_data_template_id_unknown_template_id_given() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(400).json_body(json!({
                "success": false,
                "error": "Invalid or undefined TemplateId or RenderId in the URL",
                "code": "w115"
            }));
        });

        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let json_data = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );

        let json_data = JsonData::new(json_data)?;
        let result = carbone.render_data(template_id, json_data).await;

        let expected_error = CarboneError::HttpError {
            status_code: StatusCode::BAD_REQUEST,
            error_message: "Invalid or undefined TemplateId or RenderId in the URL".to_string(),
        };

        mock_server.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_render_data_failed() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        let api_version: ApiVersion = ApiVersion::new("4".to_string())?;
        let config = Config::new("http://bad_url".to_string(), 1, api_version)?;
        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let json_data = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );

        let json_data = JsonData::new(json_data)?;
        let result = carbone.render_data(template_id, json_data).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_upload_template() -> Result<(), CarboneError> {
        let template_id_expected = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let data = APIResponseData {
            template_id: Some(template_id_expected.clone()),
            render_id: None,
            template_file_extension: None,
        };

        let body = APIResponse {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server: httpmock::Mock = server.mock(|when, then| {
            when.method("POST").path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let file_name = "template.odt";
        let file_path = format!("tests/data/{}", file_name);
        let filte_content = fs::read(file_path)?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let template_id = carbone
            .upload_template(file_name, filte_content, None)
            .await
            .unwrap();

        // Assert
        mock_server.assert();
        assert_eq!(template_id, template_id_expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_upload_template_with_payload() -> Result<(), CarboneError> {
        let template_id_expected = TemplateId::new(
            "cb03f7676ef0fbe5d7824a64676166ac2c7c789d9e6da5b7c0c46794911ee7a7".to_string(),
        )?;

        let data = APIResponseData {
            template_id: Some(template_id_expected.clone()),
            render_id: None,
            template_file_extension: None,
        };

        let body = APIResponse {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let m = server.mock(|when, then| {
            when.method("POST").path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let file_name = "template.odt";
        let file_path = format!("tests/data/{}", file_name);
        let filte_content = fs::read(file_path)?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let template_id = carbone
            .upload_template(file_name, filte_content, Some("salt1234"))
            .await
            .unwrap();

        // Assert
        m.assert();
        assert_eq!(template_id, template_id_expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_upload_template_unsupported_file_format_given() -> Result<(), CarboneError> {
        let error_msg = "Template format not supported, it must be an XML-based document: DOCX, XLSX, PPTX, ODT, ODS, ODP, XHTML, HTML or an XML file";

        let body = APIResponse {
            success: false,
            data: None,
            error: Some(error_msg.to_string()),
            code: Some("w118".to_string()),
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let m = server.mock(|when, then| {
            when.method("POST").path("/template");
            then.status(415)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let file_name = "template.odt";
        let file_path = format!("tests/data/{}", file_name);
        let filte_content = fs::read(file_path)?;

        let carbone = Carbone::new(&config, Some(&api_token))?;
        let result = carbone
            .upload_template(file_name, filte_content, None)
            .await;

        let expected_error = CarboneError::Error(error_msg.to_string());

        // Assert
        m.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_status() -> Result<(), CarboneError> {
        let body : String = "{\"success\":true,\"code\":200,\"message\":\"OK\",\"version\":\"4.22.11\"}".to_string();
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/status"));
            then.status(200).body(body.clone());
        });

        let helper = Helper::new();
        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let carbone = Carbone::new(&config, Some(&api_token))?;

        let response = carbone.get_status().await.unwrap();

        mock_server.assert();

        assert_eq!(body, response);
        Ok(())
    }
}
