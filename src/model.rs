//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub greeting: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub accesstoken: String,
    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
}