# \TransactionsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commit_transaction**](TransactionsApi.md#commit_transaction) | **PUT** /services/haproxy/transactions/{id} | Commit transaction
[**delete_transaction**](TransactionsApi.md#delete_transaction) | **DELETE** /services/haproxy/transactions/{id} | Delete a transaction
[**get_transaction**](TransactionsApi.md#get_transaction) | **GET** /services/haproxy/transactions/{id} | Return one HAProxy configuration transactions
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /services/haproxy/transactions | Return list of HAProxy configuration transactions.
[**start_transaction**](TransactionsApi.md#start_transaction) | **POST** /services/haproxy/transactions | Start a new transaction



## commit_transaction

> crate::models::Transaction commit_transaction(id, force_reload)
Commit transaction

Commit transaction, execute all operations in transaction and return msg

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Transaction id | [required] |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction

> delete_transaction(id)
Delete a transaction

Deletes a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Transaction id | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction

> crate::models::Transaction get_transaction(id)
Return one HAProxy configuration transactions

Returns one HAProxy configuration transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Transaction id | [required] |

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> Vec<crate::models::Transaction> get_transactions(status)
Return list of HAProxy configuration transactions.

Returns a list of HAProxy configuration transactions. Transactions can be filtered by their status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter by transaction status |  |

### Return type

[**Vec<crate::models::Transaction>**](transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_transaction

> crate::models::Transaction start_transaction(version)
Start a new transaction

Starts a new transaction and returns it's id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **i32** | Configuration version on which to work on | [required] |

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

