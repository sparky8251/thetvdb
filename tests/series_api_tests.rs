use futures::future::Future;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

// #[test]
// fn series_id_actor() {
//     let runtime = Runtime::new().unwrap();
//     let https = HttpsConnector::new(1).unwrap();
//     let client = hyper::client::Client::builder()
//         .executor(runtime.executor())
//         .build::<_, hyper::Body>(https);
//
//     let configuration = thetvdb::apis::configuration::Configuration::new(client);
//     let apiclient = thetvdb::apis::client::APIClient::new(configuration);
//
//     let mut apikey = thetvdb::models::Auth::default();
//     apikey.set_apikey("0629B785CE550C8D".to_string());
//
//     let token = apiclient.authentication_api().login_post(apikey);
//
//     let token = match token.wait() {
//         Ok(v) => thetvdb::apis::configuration::ApiToken {
//             token: v.token().unwrap().to_string(),
//         },
//         Err(e) => panic!("{:?}", e),
//     };
//
//     let runtime = Runtime::new().unwrap();
//     let https = HttpsConnector::new(1).unwrap();
//     let client = hyper::client::Client::builder()
//         .executor(runtime.executor())
//         .build::<_, hyper::Body>(https);
//
//     let mut configuration = thetvdb::apis::configuration::Configuration::new(client);
//     configuration.token = Some(token);
//     let apiclient = thetvdb::apis::client::APIClient::new(configuration);
//
//     let results = match apiclient
//         .series_api()
//         .series_id_actors_get(43640)
//         .wait()
//     {
//         Ok(v) => v,
//         Err(e) => panic!("{:?}", e),
//     };
//
//     let results = match results.data() {
//         Some(v) => v[0].name().unwrap(),
//         None => panic!("No data!"),
//     };
//
//     assert_eq!("Zach Braff", results)
// }

#[test]
fn series_id_episodes() {
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
        .series_api()
        .series_id_episodes_get(76156, None)
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].episode_name().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!("My First Day", results)
}

#[test]
fn series_id_episodes_page() {
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
        .series_api()
        .series_id_episodes_get(76156, Some(2))
        .wait()
    {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v[0].episode_name().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!("My Missed Perception", results)
}

#[test]
fn series_id() {
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

    let results = match apiclient.series_api().series_id_get(76156, "en").wait() {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let results = match results.data() {
        Some(v) => v.series_name().unwrap(),
        None => panic!("No data!"),
    };

    assert_eq!("Scrubs", results)
}
