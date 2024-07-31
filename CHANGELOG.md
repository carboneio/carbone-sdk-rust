# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## 2.0.0
- Released on 2024//: The package was originally made by Pascal CHENEVAS from . The Carbone team is now maintaining the SDK. This version brings all missing functions to interact with the Carbone API.
- Added function `getStatus`: It return the current status and the version of the API as `String`.
- Added error `HttpError`: It return the status code and a error message.
- Modified for the `generate_report`: Optimization of api calls when there is error 404.
- Modified for the `render_data`: When there is an error in the request, the function returns the status code and an error message.
- Added units tests.

### v1.0.0
- Released on 