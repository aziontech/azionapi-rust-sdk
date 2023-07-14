# Rust API client for personal_tokens

The Personal Tokens API allows you to manage your existing personal tokens.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `personal_tokens` and add the following to `Cargo.toml` under `[dependencies]`:

```
personal_tokens = { path = "./personal_tokens" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.azionapi.net*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*PersonalTokenApi* | [**create_personal_token**](docs/PersonalTokenApi.md#create_personal_token) | **POST** /iam/personal_tokens | Create a new personal token
*PersonalTokenApi* | [**delete_personal_token**](docs/PersonalTokenApi.md#delete_personal_token) | **DELETE** /iam/personal_tokens/{personalTokenId} | Delete a personal token by id
*PersonalTokenApi* | [**get_personal_token**](docs/PersonalTokenApi.md#get_personal_token) | **GET** /iam/personal_tokens/{personalTokenId} | Get a personal token info
*PersonalTokenApi* | [**list_personal_token**](docs/PersonalTokenApi.md#list_personal_token) | **GET** /iam/personal_tokens | List all existing personal token


## Documentation For Models

 - [CreatePersonalTokenRequest](docs/CreatePersonalTokenRequest.md)
 - [CreatePersonalTokenResponse](docs/CreatePersonalTokenResponse.md)
 - [PersonalTokenResponseGet](docs/PersonalTokenResponseGet.md)
 - [PersonalTokenResponseWithResults](docs/PersonalTokenResponseWithResults.md)
 - [PersonalTokenResponseWithResultsLinks](docs/PersonalTokenResponseWithResultsLinks.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


