# \ServiceDiscoveryApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_aws_region**](ServiceDiscoveryApi.md#create_aws_region) | **POST** /service_discovery/aws | Add a new AWS region
[**create_consul**](ServiceDiscoveryApi.md#create_consul) | **POST** /service_discovery/consul | Add a new Consul server
[**delete_aws_region**](ServiceDiscoveryApi.md#delete_aws_region) | **DELETE** /service_discovery/aws/{id} | Delete an AWS region
[**delete_consul**](ServiceDiscoveryApi.md#delete_consul) | **DELETE** /service_discovery/consul/{id} | Delete a Consul server
[**get_aws_region**](ServiceDiscoveryApi.md#get_aws_region) | **GET** /service_discovery/aws/{id} | Return an AWS region
[**get_aws_regions**](ServiceDiscoveryApi.md#get_aws_regions) | **GET** /service_discovery/aws | Return an array of all configured AWS regions
[**get_consul**](ServiceDiscoveryApi.md#get_consul) | **GET** /service_discovery/consul/{id} | Return one Consul server
[**get_consuls**](ServiceDiscoveryApi.md#get_consuls) | **GET** /service_discovery/consul | Return an array of all configured Consul servers
[**replace_aws_region**](ServiceDiscoveryApi.md#replace_aws_region) | **PUT** /service_discovery/aws/{id} | Replace an AWS region
[**replace_consul**](ServiceDiscoveryApi.md#replace_consul) | **PUT** /service_discovery/consul/{id} | Replace a Consul server



## create_aws_region

> crate::models::AwsRegion create_aws_region(aws_region)
Add a new AWS region

Add a new AWS region. Credentials are not required in case Dataplane API is running in an EC2 instance with proper IAM role attached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aws_region** | [**AwsRegion**](AwsRegion.md) |  | [required] |

### Return type

[**crate::models::AwsRegion**](awsRegion.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_consul

> crate::models::Consul create_consul(consul)
Add a new Consul server

Adds a new Consul server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consul** | [**Consul**](Consul.md) |  | [required] |

### Return type

[**crate::models::Consul**](consul.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_region

> delete_aws_region(id)
Delete an AWS region

Delete an AWS region configuration by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | AWS region ID | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_consul

> delete_consul(id)
Delete a Consul server

Deletes a Consul server configuration by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Consul server Index | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aws_region

> crate::models::GetAwsRegion200Response get_aws_region(id)
Return an AWS region

Return one AWS Region configuration by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | AWS region id | [required] |

### Return type

[**crate::models::GetAwsRegion200Response**](getAWSRegion_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aws_regions

> crate::models::GetAwsRegions200Response get_aws_regions()
Return an array of all configured AWS regions

Return all configured AWS regions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAwsRegions200Response**](getAWSRegions_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consul

> crate::models::GetConsul200Response get_consul(id)
Return one Consul server

Returns one Consul server configuration by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Consul server id | [required] |

### Return type

[**crate::models::GetConsul200Response**](getConsul_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consuls

> crate::models::GetConsuls200Response get_consuls()
Return an array of all configured Consul servers

Returns all configured Consul servers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetConsuls200Response**](getConsuls_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_aws_region

> crate::models::AwsRegion replace_aws_region(id, aws_region)
Replace an AWS region

Replace an AWS region configuration by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | AWS Region ID | [required] |
**aws_region** | [**AwsRegion**](AwsRegion.md) |  | [required] |

### Return type

[**crate::models::AwsRegion**](awsRegion.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_consul

> crate::models::Consul replace_consul(id, consul)
Replace a Consul server

Replaces a Consul server configuration by it's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Consul Index | [required] |
**consul** | [**Consul**](Consul.md) |  | [required] |

### Return type

[**crate::models::Consul**](consul.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

