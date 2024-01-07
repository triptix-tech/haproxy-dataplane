# \DiscoveryApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_endpoints**](DiscoveryApi.md#get_api_endpoints) | **GET** / | Return list of root endpoints
[**get_configuration_endpoints**](DiscoveryApi.md#get_configuration_endpoints) | **GET** /services/haproxy/configuration | Return list of HAProxy advanced configuration endpoints
[**get_haproxy_endpoints**](DiscoveryApi.md#get_haproxy_endpoints) | **GET** /services/haproxy | Return list of HAProxy related endpoints
[**get_runtime_endpoints**](DiscoveryApi.md#get_runtime_endpoints) | **GET** /services/haproxy/runtime | Return list of HAProxy advanced runtime endpoints
[**get_services_endpoints**](DiscoveryApi.md#get_services_endpoints) | **GET** /services | Return list of service endpoints
[**get_spoe_endpoints**](DiscoveryApi.md#get_spoe_endpoints) | **GET** /services/haproxy/spoe | Return list of HAProxy SPOE endpoints
[**get_stats_endpoints**](DiscoveryApi.md#get_stats_endpoints) | **GET** /services/haproxy/stats | Return list of HAProxy stats endpoints
[**get_storage_endpoints**](DiscoveryApi.md#get_storage_endpoints) | **GET** /services/haproxy/storage | Return list of HAProxy storage endpoints



## get_api_endpoints

> Vec<crate::models::Endpoint> get_api_endpoints()
Return list of root endpoints

Returns a list of root endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_endpoints

> Vec<crate::models::Endpoint> get_configuration_endpoints()
Return list of HAProxy advanced configuration endpoints

Returns a list of endpoints to be used for advanced configuration of HAProxy objects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_haproxy_endpoints

> Vec<crate::models::Endpoint> get_haproxy_endpoints()
Return list of HAProxy related endpoints

Returns a list of HAProxy related endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_endpoints

> Vec<crate::models::Endpoint> get_runtime_endpoints()
Return list of HAProxy advanced runtime endpoints

Returns a list of endpoints to be used for advanced runtime settings of HAProxy objects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services_endpoints

> Vec<crate::models::Endpoint> get_services_endpoints()
Return list of service endpoints

Returns a list of API managed services endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_endpoints

> Vec<crate::models::Endpoint> get_spoe_endpoints()
Return list of HAProxy SPOE endpoints

Returns a list of endpoints to be used for SPOE settings of HAProxy.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats_endpoints

> Vec<crate::models::Endpoint> get_stats_endpoints()
Return list of HAProxy stats endpoints

Returns a list of HAProxy stats endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_endpoints

> Vec<crate::models::Endpoint> get_storage_endpoints()
Return list of HAProxy storage endpoints

Returns a list of endpoints that use HAProxy storage for persistency, e.g. maps, ssl certificates...

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Endpoint>**](endpoint.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

