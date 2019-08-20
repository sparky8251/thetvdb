use futures::future::Future;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

#[test]
fn languages() {
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
            prefix: "Bearer".to_string(),
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

    let results = match apiclient.languages_api().languages_get().wait() {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[1].abbreviation().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!("sv", results.as_str())
}

#[test]
fn languages_id() {
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
            prefix: "Bearer".to_string(),
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

    let results = match apiclient.languages_api().languages_id_get(14).wait() {
        Ok(v) => dbg!(v),
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v.abbreviation().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!("de", results.as_str())
}
