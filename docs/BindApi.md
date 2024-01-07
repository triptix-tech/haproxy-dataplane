# \BindApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bind**](BindApi.md#create_bind) | **POST** /services/haproxy/configuration/binds | Add a new bind
[**delete_bind**](BindApi.md#delete_bind) | **DELETE** /services/haproxy/configuration/binds/{name} | Delete a bind
[**get_bind**](BindApi.md#get_bind) | **GET** /services/haproxy/configuration/binds/{name} | Return one bind
[**get_binds**](BindApi.md#get_binds) | **GET** /services/haproxy/configuration/binds | Return an array of binds
[**replace_bind**](BindApi.md#replace_bind) | **PUT** /services/haproxy/configuration/binds/{name} | Replace a bind



## create_bind

> crate::models::Bind create_bind(bind, frontend, parent_name, parent_type, transaction_id, version, force_reload)
Add a new bind

Adds a new bind in the specified frontend in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bind** | [**Bind**](Bind.md) |  | [required] |
**frontend** | Option<**String**> | Parent frontend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Bind**](bind.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bind

> delete_bind(name, frontend, parent_name, parent_type, transaction_id, version, force_reload)
Delete a bind

Deletes a bind configuration by it's name in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**frontend** | Option<**String**> | Parent frontend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
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


## get_bind

> crate::models::GetBind200Response get_bind(name, frontend, parent_name, parent_type, transaction_id)
Return one bind

Returns one bind configuration by it's name in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**frontend** | Option<**String**> | Parent frontend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBind200Response**](getBind_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_binds

> crate::models::GetBinds200Response get_binds(frontend, parent_name, parent_type, transaction_id)
Return an array of binds

Returns an array of all binds that are configured in specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend** | Option<**String**> | Parent frontend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetBinds200Response**](getBinds_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_bind

> crate::models::Bind replace_bind(name, bind, frontend, parent_name, parent_type, transaction_id, version, force_reload)
Replace a bind

Replaces a bind configuration by it's name in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Bind name | [required] |
**bind** | [**Bind**](Bind.md) |  | [required] |
**frontend** | Option<**String**> | Parent frontend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Bind**](bind.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

