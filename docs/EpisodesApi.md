# \EpisodesApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**episodes_id_get**](EpisodesApi.md#episodes_id_get) | **Get** /episodes/{id} | 


# **episodes_id_get**
> ::models::EpisodeRecordData episodes_id_get(ctx, id, optional)


Returns the full information for a given episode id. __Deprecation Warning:__ The _director_ key will be deprecated in favor of the new _directors_ key in a future release.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the episode | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the episode | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::EpisodeRecordData**](EpisodeRecordData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

