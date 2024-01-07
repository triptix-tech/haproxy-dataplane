# \FcgiAppApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_fcgi_app**](FcgiAppApi.md#create_fcgi_app) | **POST** /services/haproxy/configuration/fcgi_apps | Add an FCGI app
[**delete_fcgi_app**](FcgiAppApi.md#delete_fcgi_app) | **DELETE** /services/haproxy/configuration/fcgi_apps/{name} | Delete an FCGI app
[**get_fcgi_app**](FcgiAppApi.md#get_fcgi_app) | **GET** /services/haproxy/configuration/fcgi_apps/{name} | Return a FCGI app
[**get_fcgi_apps**](FcgiAppApi.md#get_fcgi_apps) | **GET** /services/haproxy/configuration/fcgi_apps | Return an array of FCGI apps
[**replace_fcgi_app**](FcgiAppApi.md#replace_fcgi_app) | **PUT** /services/haproxy/configuration/fcgi_apps/{name} | Replace a FCGI app



## create_fcgi_app

> crate::models::FcgiApp create_fcgi_app(fcgi_app, transaction_id, version, force_reload)
Add an FCGI app

Adds a new FCGI application to the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fcgi_app** | [**FcgiApp**](FcgiApp.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::FcgiApp**](fcgiApp.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_fcgi_app

> delete_fcgi_app(name, transaction_id, version, force_reload)
Delete an FCGI app

Deletes an FCGI application from the configuration by its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Application name | [required] |
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


## get_fcgi_app

> crate::models::GetFcgiApp200Response get_fcgi_app(name, transaction_id)
Return a FCGI app

Returns one FCGI application configuration by its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Application name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetFcgiApp200Response**](getFCGIApp_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fcgi_apps

> crate::models::GetFcgiApps200Response get_fcgi_apps(transaction_id)
Return an array of FCGI apps

Returns an array of all configured FCGI applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetFcgiApps200Response**](getFCGIApps_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_fcgi_app

> crate::models::FcgiApp replace_fcgi_app(name, fcgi_app, transaction_id, version, force_reload)
Replace a FCGI app

Replaces a FCGI application configuration by its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Application name | [required] |
**fcgi_app** | [**FcgiApp**](FcgiApp.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::FcgiApp**](fcgiApp.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

