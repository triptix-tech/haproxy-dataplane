# \SpoeTransactionsApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commit_spoe_transaction**](SpoeTransactionsApi.md#commit_spoe_transaction) | **PUT** /services/haproxy/spoe_transactions/{id} | Commit transaction
[**delete_spoe_transaction**](SpoeTransactionsApi.md#delete_spoe_transaction) | **DELETE** /services/haproxy/spoe_transactions/{id} | Delete a transaction
[**get_spoe_transaction**](SpoeTransactionsApi.md#get_spoe_transaction) | **GET** /services/haproxy/spoe_transactions/{id} | Return one SPOE configuration transactions
[**get_spoe_transactions**](SpoeTransactionsApi.md#get_spoe_transactions) | **GET** /services/haproxy/spoe_transactions | Return list of SPOE configuration transactions.
[**start_spoe_transaction**](SpoeTransactionsApi.md#start_spoe_transaction) | **POST** /services/haproxy/spoe_transactions | Start a new transaction



## commit_spoe_transaction

> crate::models::SpoeTransaction commit_spoe_transaction(spoe, id, force_reload)
Commit transaction

Commit transaction, execute all operations in transaction and return msg

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**id** | **String** | Transaction id | [required] |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::SpoeTransaction**](spoe_transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_transaction

> delete_spoe_transaction(spoe, id)
Delete a transaction

Deletes a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**id** | **String** | Transaction id | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_transaction

> crate::models::SpoeTransaction get_spoe_transaction(spoe, id)
Return one SPOE configuration transactions

Returns one SPOE configuration transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**id** | **String** | Transaction id | [required] |

### Return type

[**crate::models::SpoeTransaction**](spoe_transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_transactions

> Vec<crate::models::SpoeTransaction> get_spoe_transactions(spoe, status)
Return list of SPOE configuration transactions.

Returns a list of SPOE configuration transactions. Transactions can be filtered by their status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**status** | Option<**String**> | Filter by transaction status |  |

### Return type

[**Vec<crate::models::SpoeTransaction>**](spoe_transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_spoe_transaction

> crate::models::SpoeTransaction start_spoe_transaction(spoe, version)
Start a new transaction

Starts a new transaction and returns it's id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**version** | **i32** | Configuration version on which to work on | [required] |

### Return type

[**crate::models::SpoeTransaction**](spoe_transaction.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

