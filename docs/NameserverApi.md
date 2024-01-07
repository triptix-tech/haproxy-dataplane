# \NameserverApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nameserver**](NameserverApi.md#create_nameserver) | **POST** /services/haproxy/configuration/nameservers | Add a nameserver
[**delete_nameserver**](NameserverApi.md#delete_nameserver) | **DELETE** /services/haproxy/configuration/nameservers/{name} | Delete a nameserver
[**get_nameserver**](NameserverApi.md#get_nameserver) | **GET** /services/haproxy/configuration/nameservers/{name} | Return a nameserver
[**get_nameservers**](NameserverApi.md#get_nameservers) | **GET** /services/haproxy/configuration/nameservers | Return an array of nameservers
[**replace_nameserver**](NameserverApi.md#replace_nameserver) | **PUT** /services/haproxy/configuration/nameservers/{name} | Replace a nameserver



## create_nameserver

> crate::models::Nameserver create_nameserver(resolver, nameserver, transaction_id, version, force_reload)
Add a nameserver

Adds a new nameserver to the resolvers section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resolver** | **String** | Parent resolver name | [required] |
**nameserver** | [**Nameserver**](Nameserver.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Nameserver**](nameserver.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_nameserver

> delete_nameserver(name, resolver, transaction_id, version, force_reload)
Delete a nameserver

Deletes a nameserver from the resolvers section by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Nameserver name | [required] |
**resolver** | **String** | Parent resolver name | [required] |
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


## get_nameserver

> crate::models::GetNameserver200Response get_nameserver(name, resolver, transaction_id)
Return a nameserver

Returns one nameserver configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Nameserver name | [required] |
**resolver** | **String** | Parent resolver name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetNameserver200Response**](getNameserver_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nameservers

> crate::models::GetNameservers200Response get_nameservers(resolver, transaction_id)
Return an array of nameservers

Returns an array of all configured nameservers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resolver** | **String** | Parent resolver name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetNameservers200Response**](getNameservers_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_nameserver

> crate::models::Nameserver replace_nameserver(name, resolver, nameserver, transaction_id, version, force_reload)
Replace a nameserver

Replaces a nameserver configuration by it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Nameserver name | [required] |
**resolver** | **String** | Parent resolver name | [required] |
**nameserver** | [**Nameserver**](Nameserver.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Nameserver**](nameserver.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

