# \StickTableApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stick_table**](StickTableApi.md#get_stick_table) | **GET** /services/haproxy/runtime/stick_tables/{name} | Return Stick Table
[**get_stick_table_entries**](StickTableApi.md#get_stick_table_entries) | **GET** /services/haproxy/runtime/stick_table_entries | Return Stick Table Entries
[**get_stick_tables**](StickTableApi.md#get_stick_tables) | **GET** /services/haproxy/runtime/stick_tables | Return Stick Tables
[**set_stick_table_entries**](StickTableApi.md#set_stick_table_entries) | **POST** /services/haproxy/runtime/stick_table_entries | Set Entry to Stick Table



## get_stick_table

> crate::models::StickTable get_stick_table(name, process)
Return Stick Table

Returns one stick table from runtime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Stick table name | [required] |
**process** | **i32** | Process number if master-worker mode, if not only first process is returned | [required] |

### Return type

[**crate::models::StickTable**](stick_table.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stick_table_entries

> Vec<crate::models::StickTableEntry> get_stick_table_entries(stick_table, process, filter, key, count, offset)
Return Stick Table Entries

Returns an array of all entries in a given stick tables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stick_table** | **String** | Stick table name | [required] |
**process** | **i32** | Process number if master-worker mode, if not only first process is returned | [required] |
**filter** | Option<**String**> | A list of filters in format data.<type> <operator> <value> separated by comma |  |
**key** | Option<**String**> | Key which we want the entries for |  |
**count** | Option<**i32**> | Max number of entries to be returned for pagination |  |
**offset** | Option<**i32**> | Offset which indicates how many items we skip in pagination |  |

### Return type

[**Vec<crate::models::StickTableEntry>**](stick_table_entry.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stick_tables

> Vec<crate::models::StickTable> get_stick_tables(process)
Return Stick Tables

Returns an array of all stick tables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process** | Option<**i32**> | Process number if master-worker mode, if not all processes are returned |  |

### Return type

[**Vec<crate::models::StickTable>**](stick_table.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_stick_table_entries

> set_stick_table_entries(stick_table, process, set_stick_table_entries_request)
Set Entry to Stick Table

Create or update a stick-table entry in the table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stick_table** | **String** | Stick table name | [required] |
**process** | **i32** | Process number if master-worker mode, if not only first process is returned | [required] |
**set_stick_table_entries_request** | Option<[**SetStickTableEntriesRequest**](SetStickTableEntriesRequest.md)> | Stick table entry |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

