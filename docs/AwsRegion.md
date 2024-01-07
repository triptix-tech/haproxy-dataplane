# AwsRegion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> | AWS Access Key ID. | [optional]
**allowlist** | Option<[**Vec<crate::models::AwsFilters>**](awsFilters.md)> | Specify the AWS filters used to filter the EC2 instances to add | [optional]
**denylist** | Option<[**Vec<crate::models::AwsFilters>**](awsFilters.md)> | Specify the AWS filters used to filter the EC2 instances to ignore | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | **bool** |  | 
**id** | Option<**String**> | Auto generated ID. | [optional][readonly]
**ipv4_address** | **String** | Select which IPv4 address the Service Discovery has to use for the backend server entry | 
**name** | **String** |  | 
**region** | **String** |  | 
**retry_timeout** | **i32** | Duration in seconds in-between data pulling requests to the AWS region | 
**secret_access_key** | Option<**String**> | AWS Secret Access Key. | [optional]
**server_slots_base** | Option<**i32**> |  | [optional][default to 10]
**server_slots_growth_increment** | Option<**i32**> |  | [optional]
**server_slots_growth_type** | Option<**String**> |  | [optional][default to Exponential]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


