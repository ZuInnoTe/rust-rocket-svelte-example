# Introduction

We give here only a small excerpt of the architecture of the frontend in SvelteKit. You are encouraged to read the [Svelte documentation](https://svelte.dev/docs) and related documentation (e.g. [Svelte Material UI](https://sveltematerialui.com/)).

SvelteKit has a lot of features and it is time well-spend to read about them.

Mainly we use SvelteKit for the following reasons:

- Well-maintained framework that is lightweight and does lead to very fast light-weight frontend code.
- Rich ecosystem of third party components
- Open Source
- Other frameworks, such as React, require you to choose among many different third party components that are not always compatible or different React developers are used to different third party components

# Static assets

All static assets to be included in the frontend can be found in [../static/](../static/) (except fonts/icons).

Fonts are NPM packages from [Fontsource](https://fontsource.org/). This means they are included in the web app instead of fetching them during runtime from an external source. You need to install the fonts as devDependency, include them in [../src/routes/+layout.svelte](../src/routes/+layout.svelte) and update the scss file [../src/app.scss](../src/app.scss) with the class used for the fonts.

We have mainly a local copy of the Material icons used in the application. This is also recommended by [Google](https://developers.google.com/fonts/docs/material_icons#setup_method_2_self_hosting). This is faster and more secure than directly fetching them every time when the application us opened by th user from a Google server.

# SvelteKit

We use here [SvelteKit](https://svelte.dev/docs/kit/introduction) to make developing with Svelte simpler and it includes a lot of best practices.

# Logger

We did not define a dedicated logger yet and use the [console logging](https://developer.mozilla.org/en-US/docs/Web/API/console) functionality (e.g. console.warn, console.error).

We will provide later a dedicated logger, which makes it easier to configure logging destination beyond the console (e.g. a remote web service).

# Global HTTP Client

We provide a global HTTP Client (see [../src/lib/httpclient/](../src/lib/httpclient/)) that can be used by other components to make requests (e.g. [../src/lib/inventory/](../src/lib/inventory/)). This also has error handling and it shows to the user a [Snackbar](https://sveltematerialui.com/demo/snackbar/) with the error.

By using the HTTP Client you do not need to implement the same error handling functionality in each component that does HTTP requests.

# Accessibility

You should take care to implement [accessibility](https://svelte.dev/docs/kit/accessibility) in your application - this makes the application more usable for everyone.

Note: We had to deactivate (see [../svelte.config.js](../svelte.config.js) to activate it) one accessibility component: The route announcer. The reason was that it [clashes with Content-Security Policies](https://github.com/sveltejs/kit/issues/11993).

# Security

## Content-Security-Policy

The backend emits content security policy HTTP headers. They are very strict and aim at avoiding XSS attacks and stealing user data/credentials. This may have impact on developing the frontend. However, instead of relaxing rules you should aim at making the frontend code compliant with those secrity rules.

The application supports strict content-security-policies **_WITHOUT_** unsafe-\*. We do so by letting the backend insert every request to the frontend a random nonce (see [backend documentation](../../backend/docs/CONFIGURE.md)) into the scripts

Note: When running the frontend without the backend then no content security policies are applied. It is thus highly recommended to test all UI functionality with the backend.

## Input/Output Sanitzation

As a web frontend it is crucial that you do proper input/output sanitization in frontend AND backend. For the frontend the following mechanisms are relevant:

- By default Svelte does not render any HTML/Scripts inserted as variables or by the user: https://svelte.dev/tutorial/svelte/html-tags

You should not render HTML/Scripts in variables or by the user, because this allows simple [Cross-Site Scripting attacks](https://en.wikipedia.org/wiki/Cross-site_scripting).

Note: You always need to ADDITIONALLY do sanitization in the backend (see [documentation](../../backend/docs/ARCHITECTURE.md)).
