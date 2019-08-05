# \SearchApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_series_get**](SearchApi.md#search_series_get) | **Get** /search/series | 
[**search_series_params_get**](SearchApi.md#search_series_params_get) | **Get** /search/series/params | 


# **search_series_get**
> ::models::SeriesSearchResults search_series_get(ctx, optional)


Allows the user to search for a series based on the following parameters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| Name of the series to search for. | 
 **imdb_id** | **String**| IMDB id of the series | 
 **zap2it_id** | **String**| Zap2it ID of the series to search for. | 
 **slug** | **String**| Slug from site URL of series (https://www.thetvdb.com/series/$SLUG) | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesSearchResults**](SeriesSearchResults.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **search_series_params_get**
> ::models::EpisodeDataQueryParams search_series_params_get(ctx, )


Returns an array of parameters to query by in the `/search/series` route.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::EpisodeDataQueryParams**](EpisodeDataQueryParams.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

