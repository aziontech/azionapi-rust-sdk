# \EdgeFunctionsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_functions_get**](EdgeFunctionsApi.md#edge_functions_get) | **GET** /edge_functions | edge_functions
[**edge_functions_id_delete**](EdgeFunctionsApi.md#edge_functions_id_delete) | **DELETE** /edge_functions/{id} | edge_functions
[**edge_functions_id_get**](EdgeFunctionsApi.md#edge_functions_id_get) | **GET** /edge_functions/{id} | edge_functions
[**edge_functions_id_patch**](EdgeFunctionsApi.md#edge_functions_id_patch) | **PATCH** /edge_functions/{id} | edge_functions
[**edge_functions_id_put**](EdgeFunctionsApi.md#edge_functions_id_put) | **PUT** /edge_functions/{id} | edge_functions
[**edge_functions_post**](EdgeFunctionsApi.md#edge_functions_post) | **POST** /edge_functions | edge_functions



## edge_functions_get

> models::ListEdgeFunctionResponse edge_functions_get(page, page_size, sort, order_by)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**sort** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |

### Return type

[**models::ListEdgeFunctionResponse**](ListEdgeFunctionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_functions_id_delete

> edge_functions_id_delete(id)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_functions_id_get

> models::EdgeFunctionResponse edge_functions_id_get(id)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::EdgeFunctionResponse**](EdgeFunctionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_functions_id_patch

> models::EdgeFunctionResponse edge_functions_id_patch(id, patch_edge_function_request)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**patch_edge_function_request** | [**PatchEdgeFunctionRequest**](PatchEdgeFunctionRequest.md) |  | [required] |

### Return type

[**models::EdgeFunctionResponse**](EdgeFunctionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_functions_id_put

> models::EdgeFunctionResponse edge_functions_id_put(id, put_edge_function_request)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**put_edge_function_request** | [**PutEdgeFunctionRequest**](PutEdgeFunctionRequest.md) |  | [required] |

### Return type

[**models::EdgeFunctionResponse**](EdgeFunctionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_functions_post

> models::EdgeFunctionResponse edge_functions_post(create_edge_function_request)
edge_functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_edge_function_request** | [**CreateEdgeFunctionRequest**](CreateEdgeFunctionRequest.md) |  | [required] |

### Return type

[**models::EdgeFunctionResponse**](EdgeFunctionResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

