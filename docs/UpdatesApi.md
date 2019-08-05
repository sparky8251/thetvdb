# \UpdatesApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**updated_query_get**](UpdatesApi.md#updated_query_get) | **Get** /updated/query | 
[**updated_query_params_get**](UpdatesApi.md#updated_query_params_get) | **Get** /updated/query/params | 


# **updated_query_get**
> ::models::UpdateData updated_query_get(ctx, from_time, optional)


Returns an array of series that have changed in a maximum of one week blocks since the provided `fromTime`.   The user may specify a `toTime` to grab results for less than a week. Any timespan larger than a week will be reduced down to one week automatically.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from_time** | **String**| Epoch time to start your date range. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from_time** | **String**| Epoch time to start your date range. | 
 **to_time** | **String**| Epoch time to end your date range. Must be one week from &#x60;fromTime&#x60;. | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::UpdateData**](UpdateData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updated_query_params_get**
> ::models::UpdateDataQueryParams updated_query_params_get(ctx, )


Returns an array of valid query keys for the `/updated/query/params` route.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UpdateDataQueryParams**](UpdateDataQueryParams.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

