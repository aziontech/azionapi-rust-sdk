# \DefaultApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_delete**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_delete) | **DELETE** /edge_firewall/{edge_firewall_id}/functions_instances/{edge_function_instance_id} | Delete an Edge Functions Instance by uuid
[**edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_get**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_get) | **GET** /edge_firewall/{edge_firewall_id}/functions_instances/{edge_function_instance_id} | Retrieve an Edge Functions Instance set by uuid
[**edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_patch**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_patch) | **PATCH** /edge_firewall/{edge_firewall_id}/functions_instances/{edge_function_instance_id} | Update some Edge Functions Instance attributes
[**edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_put**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_put) | **PUT** /edge_firewall/{edge_firewall_id}/functions_instances/{edge_function_instance_id} | Overwrite some Edge Functions Instance attributes
[**edge_firewall_edge_firewall_id_functions_instances_get**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_get) | **GET** /edge_firewall/{edge_firewall_id}/functions_instances | List all user Edge Functions Instances
[**edge_firewall_edge_firewall_id_functions_instances_post**](DefaultApi.md#edge_firewall_edge_firewall_id_functions_instances_post) | **POST** /edge_firewall/{edge_firewall_id}/functions_instances | Create an Edge Functions Instance



## edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_delete

> edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_delete(edge_firewall_id, edge_function_instance_id)
Delete an Edge Functions Instance by uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**edge_function_instance_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_get

> crate::models::EdgeFunctionsInstanceResponse edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_get(edge_firewall_id, edge_function_instance_id)
Retrieve an Edge Functions Instance set by uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**edge_function_instance_id** | **i64** |  | [required] |

### Return type

[**crate::models::EdgeFunctionsInstanceResponse**](EdgeFunctionsInstanceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_patch

> crate::models::EdgeFunctionsInstanceResponse edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_patch(edge_firewall_id, edge_function_instance_id, body)
Update some Edge Functions Instance attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**edge_function_instance_id** | **i64** |  | [required] |
**body** | **crate::models::CreateEdgeFunctionsInstancesRequest** |  | [required] |

### Return type

[**crate::models::EdgeFunctionsInstanceResponse**](EdgeFunctionsInstanceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_put

> crate::models::EdgeFunctionsInstanceResponse edge_firewall_edge_firewall_id_functions_instances_edge_function_instance_id_put(edge_firewall_id, edge_function_instance_id, body)
Overwrite some Edge Functions Instance attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**edge_function_instance_id** | **i64** |  | [required] |
**body** | **crate::models::CreateEdgeFunctionsInstancesRequest** |  | [required] |

### Return type

[**crate::models::EdgeFunctionsInstanceResponse**](EdgeFunctionsInstanceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_functions_instances_get

> crate::models::ListEdgeFunctionsInstancesResponse edge_firewall_edge_firewall_id_functions_instances_get(edge_firewall_id, page, page_size, sort, order_by)
List all user Edge Functions Instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**sort** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |

### Return type

[**crate::models::ListEdgeFunctionsInstancesResponse**](ListEdgeFunctionsInstancesResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_functions_instances_post

> crate::models::EdgeFunctionsInstanceResponse edge_firewall_edge_firewall_id_functions_instances_post(edge_firewall_id, create_edge_functions_instances_request)
Create an Edge Functions Instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**create_edge_functions_instances_request** | [**CreateEdgeFunctionsInstancesRequest**](CreateEdgeFunctionsInstancesRequest.md) |  | [required] |

### Return type

[**crate::models::EdgeFunctionsInstanceResponse**](EdgeFunctionsInstanceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

