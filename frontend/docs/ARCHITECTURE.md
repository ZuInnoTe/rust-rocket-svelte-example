# Introduction

We give here only a small excerpt of the architecture of the frontend in Angular. You are encouraged to read the [Angular documentation](https://angular.dev/overview) and related documentation (e.g. [Angular Material documentation](https://material.angular.io/) or [Angular Security](https://angular.dev/best-practices/security)).

Angular has a lot of features and it is time well-spend to read about them.

Mainly we use Angular for the following reasons:

- Reliable Six-month release cycle and smaller updates in-between
- Most of the aspects needed for any complex frontend are directly available in Angular. Other frameworks, such as React, require you to choose among many different third party components that are not always compatible or different React developers are used to different third party components
- Excellent Material-based UI support using Angular material
- You can automatically migrate your Angular application and its code to newer versions of Angular using the update tool
- Open Source

# Static assets

All static assets to be included in the frontend can be found in [../src/assets/](../src/assets/). We have mainly a local copy of the Material icons used in the application. This is also recommended by [Google](https://developers.google.com/fonts/docs/material_icons#setup_method_2_self_hosting). This is faster and more secure than directly fetching them every time when the application us opened by th user from a Google server.

# Standalone Components

Since Angular 16 [Standalone components](https://angular.dev/guide/components/importing#standalone-components) have become the standard. This means each component declares their dependencies making the code easier to read and understand. Furthermore, the web page can be in certain cases faster rendered.

Hence, there is also no central app.module.ts as the dependencies are declared with the components directly.

If you want to add dependencies (e.g. logger service) centrally you can add them to [../src/main.ts](../src/main.ts).

You find all the components of the frontend in [../src/app/components/](../src/app/components/).

# Services

We use [Angular Services](https://angular.dev/guide/di) to communicate in a central place with the endpoints provided by the backend application. This avoids reloading the same data from the backend by different components.

There are some other services, such as the LoggingService, to provide a centralised way to use a logger that can be easily replaced by different logging implementations.

You find the services in [../src/app/services/](../src/app/services/).

# Global HTTP Error interceptor

We configure a global HTTP Error interceptor, so that any requests (e.g. to the backend) that fail with a HTTP Status Code indicating an error, are showing in the UI using the Angular Material Snackbar. Thus, one does not need to configure this per service reducing a lot of redundant code.

See [../src/app/services/globalerrorhandler/](../src/app/services/globalerrorhandler/).

# Logger

We provide a LoggingService centrally so you can easily change the logging implementation (e.g. instead of displaying logs in the browser console you can also send them to a backend endpoint to analzye them).

You can find an example to use the logging service in [](../src/app/components/inventory/inventory.component.ts).

You can configure the log level for the whole application in [../src/main.ts](../src/main.ts).

# Accessibility

You should take care to implement [accessibility](https://angular.dev/best-practices/a11y) in your application - this makes the application more usable for everyone.

# Security

## Content-Security-Policy

The backend emits content security policy HTTP headers. They are very strict and aim at avoiding XSS attacks and stealing user data/credentials. This may have impact on developing the frontend. However, instead of relaxing rules you should aim at making the frontend code compliant with those secrity rules.

The application supports strict content-security-policies ***WITHOUT*** unsafe-*. We do so by letting the backend insert every request to the frontend a random nonce (see [backend documentation9](../../backend/docs/CONFIGURE.md)) that is included by the frontend to protect the scripts by using Angulars [ngCspNonce](https://angular.dev/best-practices/security#content-security-policy) mechanism (first one).

For that we defined a placeholder "${cspNonce}" in [index.html](../src/index.html) that is replaced by a random nonce by the backend on every request.

## CSFR Token

The backend implements a mechanism for protecting against cross-site request forgery attacks. We configured Angular so that it can properly read and use those tokens.

See [../src/main.ts](../src/main.ts).

## Input/Output Sanitzation

As a web frontend it is crucial that you do proper input/output sanitization in frontend AND backend. For the frontend the following mechanisms are relevant:

- Angular automated [built-in sanitization](https://angular.dev/best-practices/security#sanitization-example)
- Angular Content Security Policy [Trusted Types](https://angular.dev/best-practices/security#enforcing-trusted-types). The policy is configured in the backend as Content-Security Policy. You can change it in [../../backend/src/main/resources/application.yml](../../backend/src/main/resources/application.yml) or if you use one of the example configurations [../../config/](../../config/)

Note: You always need to ADDITIONALLY do sanitization in the backend (see [documentation](../../backend/docs/ARCHITECTURE.md)).
