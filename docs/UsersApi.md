# \UsersApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_favorites_get**](UsersApi.md#user_favorites_get) | **Get** /user/favorites | 
[**user_favorites_id_delete**](UsersApi.md#user_favorites_id_delete) | **Delete** /user/favorites/{id} | 
[**user_favorites_id_put**](UsersApi.md#user_favorites_id_put) | **Put** /user/favorites/{id} | 
[**user_get**](UsersApi.md#user_get) | **Get** /user | 
[**user_ratings_get**](UsersApi.md#user_ratings_get) | **Get** /user/ratings | 
[**user_ratings_item_type_item_id_delete**](UsersApi.md#user_ratings_item_type_item_id_delete) | **Delete** /user/ratings/{itemType}/{itemId} | 
[**user_ratings_item_type_item_id_item_rating_put**](UsersApi.md#user_ratings_item_type_item_id_item_rating_put) | **Put** /user/ratings/{itemType}/{itemId}/{itemRating} | 
[**user_ratings_query_get**](UsersApi.md#user_ratings_query_get) | **Get** /user/ratings/query | 
[**user_ratings_query_params_get**](UsersApi.md#user_ratings_query_params_get) | **Get** /user/ratings/query/params | 


# **user_favorites_get**
> ::models::UserFavoritesData user_favorites_get(ctx, )


Returns an array of favorite series for a given user, will be a blank array if no favorites exist.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UserFavoritesData**](UserFavoritesData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_favorites_id_delete**
> ::models::UserFavoritesData user_favorites_id_delete(ctx, id)


Deletes the given series ID from the user’s favorite’s list and returns the updated list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 

### Return type

[**::models::UserFavoritesData**](UserFavoritesData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_favorites_id_put**
> ::models::UserFavoritesData user_favorites_id_put(ctx, id)


Adds the supplied series ID to the user’s favorite’s list and returns the updated list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| ID of the series | 

### Return type

[**::models::UserFavoritesData**](UserFavoritesData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_get**
> ::models::UserData user_get(ctx, )


Returns basic information about the currently authenticated user.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UserData**](UserData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_ratings_get**
> ::models::UserRatingsData user_ratings_get(ctx, )


Returns an array of ratings for the given user.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UserRatingsData**](UserRatingsData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_ratings_item_type_item_id_delete**
> ::models::UserRatingsDataNoLinksEmptyArray user_ratings_item_type_item_id_delete(ctx, item_type, item_id)


This route deletes a given rating of a given type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **item_type** | **String**| Item to update. Can be either &#39;series&#39;, &#39;episode&#39;, or &#39;image&#39; | 
  **item_id** | **i64**| ID of the ratings record that you wish to modify | 

### Return type

[**::models::UserRatingsDataNoLinksEmptyArray**](UserRatingsDataNoLinksEmptyArray.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_ratings_item_type_item_id_item_rating_put**
> ::models::UserRatingsDataNoLinks user_ratings_item_type_item_id_item_rating_put(ctx, item_type, item_id, item_rating)


This route updates a given rating of a given type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **item_type** | **String**| Item to update. Can be either &#39;series&#39;, &#39;episode&#39;, or &#39;image&#39; | 
  **item_id** | **i64**| ID of the ratings record that you wish to modify | 
  **item_rating** | **i64**| The updated rating number | 

### Return type

[**::models::UserRatingsDataNoLinks**](UserRatingsDataNoLinks.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_ratings_query_get**
> ::models::UserRatingsData user_ratings_query_get(ctx, optional)


Returns an array of ratings for a given user that match the query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **item_type** | **String**| Item to query. Can be either &#39;series&#39;, &#39;episode&#39;, or &#39;banner&#39; | 

### Return type

[**::models::UserRatingsData**](UserRatingsData.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **user_ratings_query_params_get**
> ::models::UserRatingsQueryParams user_ratings_query_params_get(ctx, )


Returns a list of query params for use in the `/user/ratings/query` route.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UserRatingsQueryParams**](UserRatingsQueryParams.md)

### Authorization

[jwtToken](../README.md#jwtToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

