# \BackendApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_backend**](BackendApi.md#create_backend) | **POST** /services/haproxy/configuration/backends | Add a backend
[**delete_backend**](BackendApi.md#delete_backend) | **DELETE** /services/haproxy/configuration/backends/{name} | Delete a backend
[**get_backend**](BackendApi.md#get_backend) | **GET** /services/haproxy/configuration/backends/{name} | Return a backend
[**get_backends**](BackendApi.md#get_backends) | **GET** /services/haproxy/configuration/backends | Return an array of backends
[**replace_backend**](BackendApi.md#replace_backend) | **PUT** /services/haproxy/configuration/backends/{name} | Replace a backend



## create_backend

> crate::models::Backend create_backend(backend, transaction_id, version, force_reload)
Add a backend

Adds a new backend to the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | [**Backend**](Backend.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Backend**](backend.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_backend

> delete_backend(name, transaction_id, version, force_reload)
Delete a backend

Deletes a backend from the configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Backend name | [required] |
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


## get_backend

> crate::models::GetBackend200Response get_backend(name, transaction_id)
Return a backend

Returns one backend configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBackend200Response**](getBackend_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backends

> crate::models::GetBackends200Response get_backends(transaction_id)
Return an array of backends

Returns an array of all configured backends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBackends200Response**](getBackends_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_backend

> crate::models::Backend replace_backend(name, backend, transaction_id, version, force_reload)
Replace a backend

Replaces a backend configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Backend name | [required] |
**backend** | [**Backend**](Backend.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Backend**](backend.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

