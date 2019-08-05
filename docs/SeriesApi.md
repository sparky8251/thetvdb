# \SeriesApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**series_id_actors_get**](SeriesApi.md#series_id_actors_get) | **Get** /series/{id}/actors | 
[**series_id_episodes_get**](SeriesApi.md#series_id_episodes_get) | **Get** /series/{id}/episodes | 
[**series_id_episodes_query_get**](SeriesApi.md#series_id_episodes_query_get) | **Get** /series/{id}/episodes/query | 
[**series_id_episodes_query_params_get**](SeriesApi.md#series_id_episodes_query_params_get) | **Get** /series/{id}/episodes/query/params | 
[**series_id_episodes_summary_get**](SeriesApi.md#series_id_episodes_summary_get) | **Get** /series/{id}/episodes/summary | 
[**series_id_filter_get**](SeriesApi.md#series_id_filter_get) | **Get** /series/{id}/filter | 
[**series_id_filter_params_get**](SeriesApi.md#series_id_filter_params_get) | **Get** /series/{id}/filter/params | 
[**series_id_get**](SeriesApi.md#series_id_get) | **Get** /series/{id} | 
[**series_id_head**](SeriesApi.md#series_id_head) | **Head** /series/{id} | 
[**series_id_images_get**](SeriesApi.md#series_id_images_get) | **Get** /series/{id}/images | 
[**series_id_images_query_get**](SeriesApi.md#series_id_images_query_get) | **Get** /series/{id}/images/query | 
[**series_id_images_query_params_get**](SeriesApi.md#series_id_images_query_params_get) | **Get** /series/{id}/images/query/params | 


# **series_id_actors_get**
> ::models::SeriesActors series_id_actors_get(ctx, id)


Returns actors for the given series id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 

### Return type

[**::models::SeriesActors**](SeriesActors.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_episodes_get**
> ::models::SeriesEpisodes series_id_episodes_get(ctx, id, optional)


All episodes for a given series. Paginated with 100 results per page.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **page** | **String**| Page of results to fetch. Defaults to page 1 if not provided. | 

### Return type

[**::models::SeriesEpisodes**](SeriesEpisodes.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_episodes_query_get**
> ::models::SeriesEpisodesQuery series_id_episodes_query_get(ctx, id, optional)


This route allows the user to query against episodes for the given series. The response is a paginated array of episode records.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **absolute_number** | **String**| Absolute number of the episode | 
 **aired_season** | **String**| Aired season number | 
 **aired_episode** | **String**| Aired episode number | 
 **dvd_season** | **String**| DVD season number | 
 **dvd_episode** | **String**| DVD episode number | 
 **imdb_id** | **String**| IMDB id of the series | 
 **page** | **String**| Page of results to fetch. Defaults to page 1 if not provided. | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesEpisodesQuery**](SeriesEpisodesQuery.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_episodes_query_params_get**
> ::models::SeriesEpisodesQueryParams series_id_episodes_query_params_get(ctx, id)


Returns the allowed query keys for the `/series/{id}/episodes/query` route

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 

### Return type

[**::models::SeriesEpisodesQueryParams**](SeriesEpisodesQueryParams.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_episodes_summary_get**
> ::models::SeriesEpisodesSummary series_id_episodes_summary_get(ctx, id)


Returns a summary of the episodes and seasons available for the series.  __Note__: Season \"0\" is for all episodes that are considered to be specials.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 

### Return type

[**::models::SeriesEpisodesSummary**](SeriesEpisodesSummary.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_filter_get**
> ::models::SeriesData series_id_filter_get(ctx, id, keys, optional)


Returns a series records, filtered by the supplied comma-separated list of keys. Query keys can be found at the `/series/{id}/filter/params` route.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
  **keys** | **String**| Comma-separated list of keys to filter by | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **keys** | **String**| Comma-separated list of keys to filter by | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesData**](SeriesData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_filter_params_get**
> ::models::FilterKeys series_id_filter_params_get(ctx, id, optional)


Returns the list of keys available for the `/series/{id}/filter` route

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::FilterKeys**](FilterKeys.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_get**
> ::models::SeriesData series_id_get(ctx, id, optional)


Returns a series records that contains all information known about a particular series id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesData**](SeriesData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_head**
> series_id_head(ctx, id, optional)


Returns header information only about the given series ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

 (empty response body)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_images_get**
> ::models::SeriesImagesCounts series_id_images_get(ctx, id, optional)


Returns a summary of the images for a particular series

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesImagesCounts**](SeriesImagesCounts.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_images_query_get**
> ::models::SeriesImageQueryResults series_id_images_query_get(ctx, id, optional)


Query images for the given series ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **key_type** | **String**| Type of image you&#39;re querying for (fanart, poster, etc. See ../images/query/params for more details). | 
 **resolution** | **String**| Resolution to filter by (1280x1024, for example) | 
 **sub_key** | **String**| Subkey for the above query keys. See /series/{id}/images/query/params for more information | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesImageQueryResults**](SeriesImageQueryResults.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **series_id_images_query_params_get**
> ::models::SeriesImagesQueryParams series_id_images_query_params_get(ctx, id, optional)


Returns the allowed query keys for the `/series/{id}/images/query` route. Contains a parameter record for each unique `keyType`, listing values that will return results.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i64**| ID of the series | 
 **accept_language** | **String**| Records are returned with the Episode name and Overview in the desired language, if it exists. If there is no translation for the given language, then the record is still returned but with empty values for the translated fields. | 

### Return type

[**::models::SeriesImagesQueryParams**](SeriesImagesQueryParams.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

