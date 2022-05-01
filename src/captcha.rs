mod anti_captcha;

pub struct Captcha {
    pub provider: Provider,
    pub captcha_type: CaptchaType,
    pub site_key: String,
    pub api_key: String,
}

impl Captcha {
    pub fn new(
        provider: Provider,
        captcha_type: CaptchaType,
        site_key: &str,
        api_key: &str,
    ) -> Self {
        Self {
            provider,
            captcha_type,
            site_key: site_key.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub fn solve(&self) {
        match &self.provider {
            AntiCaptcha => todo!(),
            TwoCaptcha => todo!(),
            CapMonster => todo!(),
        }
    }
}

pub enum CaptchaType {
    RecaptchaV2,
    RecaptchaV2Invisible,
    ReCaptchaV3,
    ReCaptchaV3Enterprise,
    HCaptcha,
}

pub enum Provider {
    TwoCaptcha,
    AntiCaptcha,
    CapMonster,
}
