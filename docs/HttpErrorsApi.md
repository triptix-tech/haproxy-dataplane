# \HttpErrorsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http_errors_section**](HttpErrorsApi.md#create_http_errors_section) | **POST** /services/haproxy/configuration/http_errors_sections | Add a new http-error section
[**delete_http_errors_section**](HttpErrorsApi.md#delete_http_errors_section) | **DELETE** /services/haproxy/configuration/http_errors_sections/{name} | Delete a http-error section
[**get_http_errors_section**](HttpErrorsApi.md#get_http_errors_section) | **GET** /services/haproxy/configuration/http_errors_sections/{name} | Return a http-error section
[**get_http_errors_sections**](HttpErrorsApi.md#get_http_errors_sections) | **GET** /services/haproxy/configuration/http_errors_sections | Return an array of http-error sections
[**replace_http_errors_section**](HttpErrorsApi.md#replace_http_errors_section) | **PUT** /services/haproxy/configuration/http_errors_sections/{name} | Replace a http-error section



## create_http_errors_section

> crate::models::HttpErrorsSection create_http_errors_section(http_errors_section, transaction_id, version, force_reload)
Add a new http-error section

Adds a new http-error section to the configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**http_errors_section** | [**HttpErrorsSection**](HttpErrorsSection.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpErrorsSection**](http_errors_section.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_http_errors_section

> delete_http_errors_section(name, transaction_id, version, force_reload)
Delete a http-error section

Deletes a http-error section with a given name from the configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | http-error section name | [required] |
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


## get_http_errors_section

> crate::models::GetHttpErrorsSection200Response get_http_errors_section(name, transaction_id)
Return a http-error section

Returns one http-error section with a given name from the configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | http-error section name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpErrorsSection200Response**](getHTTPErrorsSection_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_http_errors_sections

> crate::models::GetHttpErrorsSections200Response get_http_errors_sections(transaction_id)
Return an array of http-error sections

Returns an array of all configured http-error sections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetHttpErrorsSections200Response**](getHTTPErrorsSections_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_http_errors_section

> crate::models::HttpErrorsSection replace_http_errors_section(name, http_errors_section, transaction_id, version, force_reload)
Replace a http-error section

Replaces a http-error section with a given name in the configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | http-error section name | [required] |
**http_errors_section** | [**HttpErrorsSection**](HttpErrorsSection.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::HttpErrorsSection**](http_errors_section.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

