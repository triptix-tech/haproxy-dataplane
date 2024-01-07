# \StatsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stats**](StatsApi.md#get_stats) | **GET** /services/haproxy/stats/native | Gets stats



## get_stats

> Vec<crate::models::NativeStatsCollection> get_stats(r#type, name, parent)
Gets stats

Getting stats from the HAProxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Object type to get stats for (one of frontend, backend, server) |  |
**name** | Option<**String**> | Object name to get stats for |  |
**parent** | Option<**String**> | Object parent name to get stats for, in case the object is a server |  |

### Return type

[**Vec<crate::models::NativeStatsCollection>**](native_stats_collection.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

