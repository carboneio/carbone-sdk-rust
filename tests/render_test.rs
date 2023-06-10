use httpmock::prelude::*;

use carbone_sdk_rs::render::Render;
use carbone_sdk_rs::errors::CarboneSdkError;

use serde_json::json;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::*;

    #[test]
    fn test_render_report() -> Result<(), CarboneSdkError> {

        let helper = Helper::new();

        let template_id = TemplateId::new("foiejwoi21e093ru3209jf2093j".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let expected_render_id = "MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string();
        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200)
                .json_body(json!({
                    "success": true,
                    "data": {
                        "renderId": expected_render_id.clone(),
                        "inputFileExtension": "odt"
                    }
                }));
        });

        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let render = Render::new(config, api_token);
        
        let render_options = String::from(r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#);
        let resp = render.render_report(template_id, render_options)?;

        mock_server.assert();
        assert_eq!(resp, expected_render_id);
        Ok(())
    }
}