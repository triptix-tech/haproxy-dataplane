# \ClusterApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_cluster**](ClusterApi.md#delete_cluster) | **DELETE** /cluster | Delete cluster settings
[**edit_cluster**](ClusterApi.md#edit_cluster) | **PUT** /cluster | Edit cluster settings
[**get_cluster**](ClusterApi.md#get_cluster) | **GET** /cluster | Return cluster data
[**initiate_certificate_refresh**](ClusterApi.md#initiate_certificate_refresh) | **POST** /cluster/certificate | Initiates a certificate refresh
[**post_cluster**](ClusterApi.md#post_cluster) | **POST** /cluster | Post cluster settings



## delete_cluster

> delete_cluster(configuration, version)
Delete cluster settings

Delete cluster settings and move the node back to single mode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration** | Option<**String**> | In case of moving to single mode do we keep or clean configuration |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_cluster

> crate::models::ClusterSettings edit_cluster(cluster_settings, version)
Edit cluster settings

Edit cluster settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_settings** | [**ClusterSettings**](ClusterSettings.md) |  | [required] |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::ClusterSettings**](cluster_settings.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> crate::models::ClusterSettings get_cluster()
Return cluster data

Returns cluster data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClusterSettings**](cluster_settings.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_certificate_refresh

> initiate_certificate_refresh()
Initiates a certificate refresh

Initiates a certificate refresh

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cluster

> crate::models::ClusterSettings post_cluster(cluster_settings, configuration, advertised_address, advertised_port, version)
Post cluster settings

Post cluster settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_settings** | [**ClusterSettings**](ClusterSettings.md) |  | [required] |
**configuration** | Option<**String**> | In case of moving to single mode do we keep or clean configuration |  |
**advertised_address** | Option<**String**> | Force the advertised address when joining a cluster |  |
**advertised_port** | Option<**i32**> | Force the advertised port when joining a cluster |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::ClusterSettings**](cluster_settings.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

