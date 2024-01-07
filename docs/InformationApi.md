# \InformationApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_haproxy_process_info**](InformationApi.md#get_haproxy_process_info) | **GET** /services/haproxy/runtime/info | Return HAProxy process information
[**get_info**](InformationApi.md#get_info) | **GET** /info | Return API, hardware and OS information



## get_haproxy_process_info

> Vec<crate::models::ProcessInfo> get_haproxy_process_info()
Return HAProxy process information

Return HAProxy process information

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ProcessInfo>**](process_info.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_info

> crate::models::Info get_info()
Return API, hardware and OS information

Return API, hardware and OS information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Info**](info.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

