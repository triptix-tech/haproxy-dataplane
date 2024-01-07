# \HttpCheckApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_check**](HttpCheckApi.md#create_http_check) | **POST** /services/haproxy/configuration/http_checks | Add a new HTTP check
[**delete_http_check**](HttpCheckApi.md#delete_http_check) | **DELETE** /services/haproxy/configuration/http_checks/{index} | Delete a HTTP check
[**get_http_check**](HttpCheckApi.md#get_http_check) | **GET** /services/haproxy/configuration/http_checks/{index} | Return one HTTP check
[**get_http_checks**](HttpCheckApi.md#get_http_checks) | **GET** /services/haproxy/configuration/http_checks | Return an array of HTTP checks
[**replace_http_check**](HttpCheckApi.md#replace_http_check) | **PUT** /services/haproxy/configuration/http_checks/{index} | Replace a HTTP check



## create_http_check

> crate::models::HttpCheck create_http_check(parent_type, http_check, parent_name, transaction_id, version, force_reload)
Add a new HTTP check

Adds a new HTTP check of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**http_check** | [**HttpCheck**](HttpCheck.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpCheck**](http_check.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_check

> delete_http_check(index, parent_type, parent_name, transaction_id, version, force_reload)
Delete a HTTP check

Deletes a HTTP check configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP check Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
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


## get_http_check

> crate::models::GetHttpCheck200Response get_http_check(index, parent_type, parent_name, transaction_id)
Return one HTTP check

Returns one HTTP check configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Check Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpCheck200Response**](getHTTPCheck_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_checks

> crate::models::GetHttpChecks200Response get_http_checks(parent_type, parent_name, transaction_id)
Return an array of HTTP checks

Returns all HTTP checks that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpChecks200Response**](getHTTPChecks_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_check

> crate::models::HttpCheck replace_http_check(index, parent_type, http_check, parent_name, transaction_id, version, force_reload)
Replace a HTTP check

Replaces a HTTP Check configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | HTTP Check Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**http_check** | [**HttpCheck**](HttpCheck.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpCheck**](http_check.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

