# \LogTargetApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_target**](LogTargetApi.md#create_log_target) | **POST** /services/haproxy/configuration/log_targets | Add a new Log Target
[**delete_log_target**](LogTargetApi.md#delete_log_target) | **DELETE** /services/haproxy/configuration/log_targets/{index} | Delete a Log Target
[**get_log_target**](LogTargetApi.md#get_log_target) | **GET** /services/haproxy/configuration/log_targets/{index} | Return one Log Target
[**get_log_targets**](LogTargetApi.md#get_log_targets) | **GET** /services/haproxy/configuration/log_targets | Return an array of all Log Targets
[**replace_log_target**](LogTargetApi.md#replace_log_target) | **PUT** /services/haproxy/configuration/log_targets/{index} | Replace a Log Target



## create_log_target

> crate::models::LogTarget create_log_target(parent_type, log_target, parent_name, transaction_id, version, force_reload)
Add a new Log Target

Adds a new Log Target of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**log_target** | [**LogTarget**](LogTarget.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::LogTarget**](log_target.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_log_target

> delete_log_target(index, parent_type, parent_name, transaction_id, version, force_reload)
Delete a Log Target

Deletes a Log Target configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Log Target Index | [required] |
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


## get_log_target

> crate::models::GetLogTarget200Response get_log_target(index, parent_type, parent_name, transaction_id)
Return one Log Target

Returns one Log Target configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Log Target Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetLogTarget200Response**](getLogTarget_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_targets

> crate::models::GetLogTargets200Response get_log_targets(parent_type, parent_name, transaction_id)
Return an array of all Log Targets

Returns all Log Targets that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetLogTargets200Response**](getLogTargets_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_log_target

> crate::models::LogTarget replace_log_target(index, parent_type, log_target, parent_name, transaction_id, version, force_reload)
Replace a Log Target

Replaces a Log Target configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Log Target Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**log_target** | [**LogTarget**](LogTarget.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::LogTarget**](log_target.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

