# \DeclareCaptureApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_declare_capture**](DeclareCaptureApi.md#create_declare_capture) | **POST** /services/haproxy/configuration/captures | Add a new declare capture
[**delete_declare_capture**](DeclareCaptureApi.md#delete_declare_capture) | **DELETE** /services/haproxy/configuration/captures/{index} | Delete a declare capture
[**get_declare_capture**](DeclareCaptureApi.md#get_declare_capture) | **GET** /services/haproxy/configuration/captures/{index} | Return one declare capture
[**get_declare_captures**](DeclareCaptureApi.md#get_declare_captures) | **GET** /services/haproxy/configuration/captures | Return an array of declare captures
[**replace_declare_capture**](DeclareCaptureApi.md#replace_declare_capture) | **PUT** /services/haproxy/configuration/captures/{index} | Replace a declare capture



## create_declare_capture

> crate::models::Capture create_declare_capture(frontend, capture, transaction_id, version, force_reload)
Add a new declare capture

Adds a new declare capture in the specified frontend in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend** | **String** | Parent frontend name | [required] |
**capture** | [**Capture**](Capture.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Capture**](capture.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_declare_capture

> delete_declare_capture(index, frontend, transaction_id, version, force_reload)
Delete a declare capture

Deletes a declare capture configuration by it's index in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Declare Capture Index | [required] |
**frontend** | **String** | Parent frontend name | [required] |
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


## get_declare_capture

> crate::models::GetDeclareCapture200Response get_declare_capture(index, frontend, transaction_id)
Return one declare capture

Returns one declare capture configuration by it's index in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Declare Capture Index | [required] |
**frontend** | **String** | Parent frontend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetDeclareCapture200Response**](getDeclareCapture_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_declare_captures

> crate::models::GetDeclareCaptures200Response get_declare_captures(frontend, transaction_id)
Return an array of declare captures

Returns an array of all declare capture records that are configured in specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend** | **String** | Parent frontend name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetDeclareCaptures200Response**](getDeclareCaptures_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_declare_capture

> crate::models::Capture replace_declare_capture(index, frontend, capture, transaction_id, version, force_reload)
Replace a declare capture

Replaces a declare capture configuration by it's index in the specified frontend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** | Declare Capture Index | [required] |
**frontend** | **String** | Parent frontend name | [required] |
**capture** | [**Capture**](Capture.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Capture**](capture.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

