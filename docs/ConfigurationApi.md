# \ConfigurationApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration_version**](ConfigurationApi.md#get_configuration_version) | **GET** /services/haproxy/configuration/version | Return a configuration version
[**get_ha_proxy_configuration**](ConfigurationApi.md#get_ha_proxy_configuration) | **GET** /services/haproxy/configuration/raw | Return HAProxy configuration
[**post_ha_proxy_configuration**](ConfigurationApi.md#post_ha_proxy_configuration) | **POST** /services/haproxy/configuration/raw | Push new haproxy configuration



## get_configuration_version

> i32 get_configuration_version(transaction_id)
Return a configuration version

Returns configuration version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

**i32**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ha_proxy_configuration

> crate::models::GetHaProxyConfiguration200Response get_ha_proxy_configuration(transaction_id, version)
Return HAProxy configuration

Returns HAProxy configuration file in plain text

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::GetHaProxyConfiguration200Response**](getHAProxyConfiguration_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ha_proxy_configuration

> String post_ha_proxy_configuration(body, skip_version, skip_reload, only_validate, x_runtime_actions, version, force_reload)
Push new haproxy configuration

Push a new haproxy configuration file in plain text

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**skip_version** | Option<**bool**> | If set, no version check will be done and the pushed config will be enforced |  |[default to false]
**skip_reload** | Option<**bool**> | If set, no reload will be initiated and runtime actions from X-Runtime-Actions will be applied |  |[default to false]
**only_validate** | Option<**bool**> | If set, only validates configuration, without applying it |  |[default to false]
**x_runtime_actions** | Option<**String**> | List of Runtime API commands with parameters separated by ';' |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

**String**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

