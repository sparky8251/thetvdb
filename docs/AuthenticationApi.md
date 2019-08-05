# \AuthenticationApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login_post**](AuthenticationApi.md#login_post) | **Post** /login | 
[**refresh_token_get**](AuthenticationApi.md#refresh_token_get) | **Get** /refresh_token | 


# **login_post**
> ::models::Token login_post(authentication_string)


Returns a session token to be included in the rest of the requests. Note that API key authentication is required for all subsequent requests and user auth is required for routes in the `User` section

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **authentication_string** | [**Auth**](Auth.md)| JSON string containing your authentication details. | 

### Return type

[**::models::Token**](Token.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refresh_token_get**
> ::models::Token refresh_token_get(ctx, )


Refreshes your current, valid JWT token and returns a new token. Hit this route so that you do not have to post to `/login` with your API key and credentials once you have already been authenticated.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Token**](Token.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

