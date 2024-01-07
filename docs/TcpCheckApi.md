# \TcpCheckApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcp_check**](TcpCheckApi.md#create_tcp_check) | **POST** /services/haproxy/configuration/tcp_checks | Add a new TCP check
[**delete_tcp_check**](TcpCheckApi.md#delete_tcp_check) | **DELETE** /services/haproxy/configuration/tcp_checks/{index} | Delete a TCP check
[**get_tcp_check**](TcpCheckApi.md#get_tcp_check) | **GET** /services/haproxy/configuration/tcp_checks/{index} | Return one TCP check
[**get_tcp_checks**](TcpCheckApi.md#get_tcp_checks) | **GET** /services/haproxy/configuration/tcp_checks | Return an array of TCP checks
[**replace_tcp_check**](TcpCheckApi.md#replace_tcp_check) | **PUT** /services/haproxy/configuration/tcp_checks/{index} | Replace a TCP check



## create_tcp_check

> crate::models::TcpCheck create_tcp_check(parent_type, tcp_check, parent_name, transaction_id, version, force_reload)
Add a new TCP check

Adds a new TCP check of the specified type in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**tcp_check** | [**TcpCheck**](TcpCheck.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpCheck**](tcp_check.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tcp_check

> delete_tcp_check(index, parent_type, parent_name, transaction_id, version, force_reload)
Delete a TCP check

Deletes a TCP check configuration by it's index from the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP check Index | [required] |
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


## get_tcp_check

> crate::models::GetTcpCheck200Response get_tcp_check(index, parent_type, parent_name, transaction_id)
Return one TCP check

Returns one TCP check configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Check Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpCheck200Response**](getTCPCheck_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tcp_checks

> crate::models::GetTcpChecks200Response get_tcp_checks(parent_type, parent_name, transaction_id)
Return an array of TCP checks

Returns all TCP checks that are configured in specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | **String** | Parent type | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetTcpChecks200Response**](getTCPChecks_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_tcp_check

> crate::models::TcpCheck replace_tcp_check(index, parent_type, tcp_check, parent_name, transaction_id, version, force_reload)
Replace a TCP check

Replaces a TCP Check configuration by it's index in the specified parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | TCP Check Index | [required] |
**parent_type** | **String** | Parent type | [required] |
**tcp_check** | [**TcpCheck**](TcpCheck.md) |  | [required] |
**parent_name** | Option<**String**> | Parent name |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::TcpCheck**](tcp_check.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

