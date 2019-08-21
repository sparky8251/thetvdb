use futures::future::Future;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

#[test]
fn search_series_name() {
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

    let results = match apiclient
        .search_api()
        .search_series_get(thetvdb::models::SeriesSearchQueryParams::Name("scrubs".to_string()), "en")
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].id().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!(76156, *results)
}

#[test]
fn search_series_imdbid() {
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

    let results = match apiclient
        .search_api()
        .search_series_get(thetvdb::models::SeriesSearchQueryParams::ImdbId("tt0285403".to_string()), "en")
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].id().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!(76156, *results)
}

#[test]
fn search_series_zap2itid() {
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

    let results = match apiclient
        .search_api()
        .search_series_get(thetvdb::models::SeriesSearchQueryParams::Zap2ItId("EP00446160".to_string()), "en")
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].id().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!(76156, *results)
}

#[test]
fn search_series_slug() {
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

    let results = match apiclient
        .search_api()
        .search_series_get(thetvdb::models::SeriesSearchQueryParams::Slug("scrubs".to_string()), "en")
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].id().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!(76156, *results)
}
