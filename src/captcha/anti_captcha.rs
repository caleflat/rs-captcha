use super::Captcha;
use super::CaptchaType;

pub fn solve(captcha: Captcha) -> Result<String, String> {
    if let err = solve(captcha).unwrap_err() {
        return Err(err);
    }

    Ok("".to_string())
}

async fn create_task(captcha: Captcha) -> Result<String, String> {
    let payload = serde_json::json!({
        "clientKey": captcha.api_key,
        "task": match captcha.captcha_type {
            CaptchaType::RecaptchaV2 => "RecaptchaV2TaskProxyless",
            CaptchaType::RecaptchaV2Invisible => return Err("Not Supported".to_string()),
            CaptchaType::ReCaptchaV3 => "RecaptchaV3TaskProxyless",
            CaptchaType::ReCaptchaV3Enterprise => "RecaptchaV2EnterpriseTaskProxyless",
            CaptchaType::HCaptcha => "HCaptchaTaskProxyless"
        }
    });

    let client = reqwest::Client::new();
    let res = match client
        .post("https://api.anti-captcha.com/createTask")
        .json(&payload)
        .send()
        .await {
            Ok(res) => res,
            Err(_) => return Err("Error creating task".to_string())
        };

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        #[serde()]
        error_id: u32,
        error_code: Option<String>,
        error_description: Option<String>,
        task_id: Option<u32>,
    }
    
    let response_json = match res.json::<Response>().await {
        Ok(j) => j,
        Err(_) => return Err("error unmarshalling response".to_string())
    };

    if response_json.error_id != 0 {
        return Err(format!("Error creating task: {:?}", response_json.error_description))
    }

    if let Some(task_id) = response_json.task_id {
        Ok(&task_id.to_string())
    }

    Err("Error creating task".to_string())
}
