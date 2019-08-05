# \LanguagesApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**languages_get**](LanguagesApi.md#languages_get) | **Get** /languages | 
[**languages_id_get**](LanguagesApi.md#languages_id_get) | **Get** /languages/{id} | 


# **languages_get**
> ::models::LanguageData languages_get(ctx, )


All available languages. These language abbreviations can be used in the `Accept-Language` header for routes that return translation records.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::LanguageData**](LanguageData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **languages_id_get**
> ::models::Language languages_id_get(ctx, id)


Information about a particular language, given the language ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of the language | 

### Return type

[**::models::Language**](Language.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

