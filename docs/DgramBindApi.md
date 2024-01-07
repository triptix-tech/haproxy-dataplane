# \DgramBindApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dgram_bind**](DgramBindApi.md#create_dgram_bind) | **POST** /services/haproxy/configuration/dgram_binds | Add a new dgram bind
[**delete_dgram_bind**](DgramBindApi.md#delete_dgram_bind) | **DELETE** /services/haproxy/configuration/dgram_binds/{name} | Delete a dgram bind
[**get_dgram_bind**](DgramBindApi.md#get_dgram_bind) | **GET** /services/haproxy/configuration/dgram_binds/{name} | Return one dgram bind
[**get_dgram_binds**](DgramBindApi.md#get_dgram_binds) | **GET** /services/haproxy/configuration/dgram_binds | Return an array of dgram binds
[**replace_dgram_bind**](DgramBindApi.md#replace_dgram_bind) | **PUT** /services/haproxy/configuration/dgram_binds/{name} | Replace a dgram bind



## create_dgram_bind

> crate::models::DgramBind create_dgram_bind(log_forward, dgram_bind, transaction_id, version, force_reload)
Add a new dgram bind

Adds a new dgram bind in the specified log forward in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_forward** | **String** | Parent log forward name | [required] |
**dgram_bind** | [**DgramBind**](DgramBind.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::DgramBind**](dgram_bind.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dgram_bind

> delete_dgram_bind(name, log_forward, transaction_id, version, force_reload)
Delete a dgram bind

Deletes a dgram bind configuration by it's name in the specified log forward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**log_forward** | **String** | Parent log forward name | [required] |
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


## get_dgram_bind

> crate::models::GetDgramBind200Response get_dgram_bind(name, log_forward, transaction_id)
Return one dgram bind

Returns one dgram bind configuration by it's name in the specified log forward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**log_forward** | **String** | Parent log forward name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetDgramBind200Response**](getDgramBind_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dgram_binds

> crate::models::GetDgramBinds200Response get_dgram_binds(log_forward, transaction_id)
Return an array of dgram binds

Returns an array of all dgram binds that are configured in specified log forward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_forward** | **String** | Parent log forward name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetDgramBinds200Response**](getDgramBinds_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_dgram_bind

> crate::models::DgramBind replace_dgram_bind(name, log_forward, dgram_bind, transaction_id, version, force_reload)
Replace a dgram bind

Replaces a dgram bind configuration by it's name in the specified log forward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**log_forward** | **String** | Parent log forward name | [required] |
**dgram_bind** | [**DgramBind**](DgramBind.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::DgramBind**](dgram_bind.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

