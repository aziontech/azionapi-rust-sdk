# \PersonalTokenApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_personal_token**](PersonalTokenApi.md#create_personal_token) | **POST** /iam/personal_tokens | Create a new personal token
[**delete_personal_token**](PersonalTokenApi.md#delete_personal_token) | **DELETE** /iam/personal_tokens/{personalTokenId} | Delete a personal token by id
[**get_personal_token**](PersonalTokenApi.md#get_personal_token) | **GET** /iam/personal_tokens/{personalTokenId} | Get a personal token info
[**list_personal_token**](PersonalTokenApi.md#list_personal_token) | **GET** /iam/personal_tokens | List all existing personal token



## create_personal_token

> crate::models::CreatePersonalTokenResponse create_personal_token(create_personal_token_request)
Create a new personal token

Create a new personal token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_personal_token_request** | [**CreatePersonalTokenRequest**](CreatePersonalTokenRequest.md) |  | [required] |

### Return type

[**crate::models::CreatePersonalTokenResponse**](CreatePersonalTokenResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_personal_token

> delete_personal_token(personal_token_id)
Delete a personal token by id

Delete a personal token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personal_token_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_personal_token

> crate::models::PersonalTokenResponseGet get_personal_token(personal_token_id)
Get a personal token info

Get a personal token info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personal_token_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::PersonalTokenResponseGet**](PersonalTokenResponseGet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_personal_token

> crate::models::PersonalTokenResponseWithResults list_personal_token()
List all existing personal token

List all existing personal token

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PersonalTokenResponseWithResults**](PersonalTokenResponseWithResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

