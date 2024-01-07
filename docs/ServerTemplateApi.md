# \ServerTemplateApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server_template**](ServerTemplateApi.md#create_server_template) | **POST** /services/haproxy/configuration/server_templates | Add a new server template
[**delete_server_template**](ServerTemplateApi.md#delete_server_template) | **DELETE** /services/haproxy/configuration/server_templates/{prefix} | Delete a server template
[**get_server_template**](ServerTemplateApi.md#get_server_template) | **GET** /services/haproxy/configuration/server_templates/{prefix} | Return one server template
[**get_server_templates**](ServerTemplateApi.md#get_server_templates) | **GET** /services/haproxy/configuration/server_templates | Return an array of server templates
[**replace_server_template**](ServerTemplateApi.md#replace_server_template) | **PUT** /services/haproxy/configuration/server_templates/{prefix} | Replace a server template



## create_server_template

> crate::models::ServerTemplate create_server_template(backend, server_template, transaction_id, version, force_reload)
Add a new server template

Adds a new server template in the specified backend in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |
**server_template** | [**ServerTemplate**](ServerTemplate.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::ServerTemplate**](server_template.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server_template

> delete_server_template(prefix, backend, transaction_id, version, force_reload)
Delete a server template

Deletes a server template configuration by it's prefix in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** | Server template prefix | [required] |
**backend** | **String** | Parent backend name | [required] |
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


## get_server_template

> crate::models::GetServerTemplate200Response get_server_template(prefix, backend, transaction_id)
Return one server template

Returns one server template configuration by it's prefix in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** | Server template prefix | [required] |
**backend** | **String** | Parent backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServerTemplate200Response**](getServerTemplate_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_templates

> crate::models::GetServerTemplates200Response get_server_templates(backend, transaction_id)
Return an array of server templates

Returns an array of all server templates that are configured in specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServerTemplates200Response**](getServerTemplates_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_server_template

> crate::models::ServerTemplate replace_server_template(prefix, backend, server_template, transaction_id, version, force_reload)
Replace a server template

Replaces a server template configuration by it's prefix in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** | Server template prefix | [required] |
**backend** | **String** | Parent backend name | [required] |
**server_template** | [**ServerTemplate**](ServerTemplate.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::ServerTemplate**](server_template.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

