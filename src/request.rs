use reqwest::StatusCode;

use crate::types::osrs_api::OsrsApiErr;

const OSRS_API: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json";

pub fn get_user_data(rsn: String) -> Result<String, OsrsApiErr> {
    // let res = reqwest::blocking::get(format!("{OSRS_API}?player={rsn}"))?.text()?;
    let res = reqwest::blocking::get(format!("{OSRS_API}?player={rsn}"))?;
    let status: StatusCode = res.status();

    match status {
        StatusCode::OK                                            => Ok(res.text()?),
        StatusCode::NOT_FOUND                                     => Err(OsrsApiErr::UserNotFound),
        StatusCode::BAD_REQUEST                                   => Err(OsrsApiErr::InvalidRequest),
        StatusCode::SERVICE_UNAVAILABLE | StatusCode::BAD_GATEWAY => Err(OsrsApiErr::ApiIssue),
        _                                                         => Err(OsrsApiErr::UnknownIssue)
    }
}