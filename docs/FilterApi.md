# \FilterApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_filter**](FilterApi.md#create_filter) | **POST** /services/haproxy/configuration/filters | Add a new Filter
[**delete_filter**](FilterApi.md#delete_filter) | **DELETE** /services/haproxy/configuration/filters/{index} | Delete a Filter
[**get_filter**](FilterApi.md#get_filter) | **GET** /services/haproxy/configuration/filters/{index} | Return one Filter
[**get_filters**](FilterApi.md#get_filters) | **GET** /services/haproxy/configuration/filters | Return an array of all Filters
[**replace_filter**](FilterApi.md#replace_filter) | **PUT** /services/haproxy/configuration/filters/{index} | Replace a Filter



## create_filter

> crate::models::Filter create_filter(parent_name, parent_type, filter, transaction_id, version, force_reload)
Add a new Filter

Adds a new Filter of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**filter** | [**Filter**](Filter.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Filter**](filter.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_filter

> delete_filter(index, parent_name, parent_type, transaction_id, version, force_reload)
Delete a Filter

Deletes a Filter configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Filter Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filter

> crate::models::GetFilter200Response get_filter(index, parent_name, parent_type, transaction_id)
Return one Filter

Returns one Filter configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Filter Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetFilter200Response**](getFilter_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filters

> crate::models::GetFilters200Response get_filters(parent_name, parent_type, transaction_id)
Return an array of all Filters

Returns all Filters that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetFilters200Response**](getFilters_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_filter

> crate::models::Filter replace_filter(index, parent_name, parent_type, filter, transaction_id, version, force_reload)
Replace a Filter

Replaces a Filter configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Filter Index | [required] |
**parent_name** | **String** | Parent name | [required] |
**parent_type** | **String** | Parent type | [required] |
**filter** | [**Filter**](Filter.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Filter**](filter.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

