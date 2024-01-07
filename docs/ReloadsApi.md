# \ReloadsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_reload**](ReloadsApi.md#get_reload) | **GET** /services/haproxy/reloads/{id} | Return one HAProxy reload status
[**get_reloads**](ReloadsApi.md#get_reloads) | **GET** /services/haproxy/reloads | Return list of HAProxy Reloads.



## get_reload

> crate::models::Reload get_reload(id)
Return one HAProxy reload status

Returns one HAProxy reload status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Reload id | [required] |

### Return type

[**crate::models::Reload**](reload.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reloads

> Vec<crate::models::Reload> get_reloads()
Return list of HAProxy Reloads.

Returns a list of HAProxy reloads.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Reload>**](reload.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

