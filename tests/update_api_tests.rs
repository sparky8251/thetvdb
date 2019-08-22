use futures::future::Future;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

#[test]
fn updates() {
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

    let results = match apiclient
        .updates_api()
        .updated_query_get(
            thetvdb::models::UpdateDataQueryParams::new(1_566_259_200, Some(1_566_950_400)),
            "en",
        )
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    match results.data() {
        Some(_) => (),
        None => panic!("No data!"),
    }
}
