use futures::future::Future;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

#[test]
fn login() {
    let runtime = Runtime::new().unwrap();
    let https = HttpsConnector::new(1).unwrap();
    let client = hyper::client::Client::builder()
        .executor(runtime.executor())
        .build::<_, hyper::Body>(https);

    let configuration = thetvdb::apis::configuration::Configuration::new(client);
    let apiclient = thetvdb::apis::client::APIClient::new(configuration);

    let mut apikey = thetvdb::models::Auth::default();
    apikey.set_apikey("0629B785CE550C8D".to_string());

    let token = apiclient.authentication_api().login_post(apikey);

    match token.wait() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
}

#[test]
fn refresh_token() {
    let runtime = Runtime::new().unwrap();
    let https = HttpsConnector::new(1).unwrap();
    let client = hyper::client::Client::builder()
        .executor(runtime.executor())
        .build::<_, hyper::Body>(https);

    let configuration = thetvdb::apis::configuration::Configuration::new(client);
    let apiclient = thetvdb::apis::client::APIClient::new(configuration);

    let mut apikey = thetvdb::models::Auth::default();
    apikey.set_apikey("0629B785CE550C8D".to_string());

    let token = apiclient.authentication_api().login_post(apikey);

    let token = match token.wait() {
        Ok(v) => thetvdb::apis::configuration::ApiToken {
            token: v.token().unwrap().to_string(),
        },
        Err(e) => panic!("{:?}", e),
    };

    let runtime = Runtime::new().unwrap();
    let https = HttpsConnector::new(1).unwrap();
    let client = hyper::client::Client::builder()
        .executor(runtime.executor())
        .build::<_, hyper::Body>(https);

    let mut configuration = thetvdb::apis::configuration::Configuration::new(client);
    configuration.token = Some(token);
    let apiclient = thetvdb::apis::client::APIClient::new(configuration);

    let token = apiclient.authentication_api().refresh_token_get();

    match token.wait() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}
