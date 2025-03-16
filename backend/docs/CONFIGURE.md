# Introduction
[Rocket allows configuration](https://rocket.rs/guide/configuration/#configuration) using:
* Rocket.toml
* Enviroment variables

Additionally, it allows [application-specific configuration](https://rocket.rs/guide/configuration/#extracting-values) using the same mechanism.

We added some application-specific configuration (see below). You find in [./ARCHITECTURE.md](./ARCHITECTURE.md) more details on how this is implemented.


Rocket allows also to customize the file and environment variables using [custom providers](https://rocket.rs/guide/configuration/#custom-providers), but we currently do not use it in this example. 

# Rocket
Rocket allows to [configure various functionality](https://rocket.rs/guide/configuration/#overview).

See [../Rocket.toml](../Rocket.toml) for an example.

## Logging

See the Rocket documentation to configure the [log level](https://rocket.rs/guide/configuration/#overview).

## Secret
We leverage the Rocket secret functionality to encrypt confidential information (e.g. OIDC Token of the user) in cookies.

Hence, you must set it when deploying the application in production.

See the [Rocket documentation](https://rocket.rs/guide/configuration/#secret-key) on how to generate a good secret. It makes also sense to frequently change it.

## IPv6
We configured in [../Rocket.toml](../Rocket.toml) to listen on the IPv6 localhost (::1) as IPv6 should be the default IP for any web application. If you want to publish the server you need to configure "::" to listen to any IPv6 address.

A production application should use port 443 for encrypted commuication.

## HTTP
We enabled in [../Cargo.toml](../Cargo.toml) the HTTP2 protocol. HTTP3 will be supported in future Rocket versions and you [should consider to support it](https://zuinnote.eu/blog/?p=2839) as well.

## TLS
An application in production will need to have a valid certificate. If you publish the application on the public Internet (see also [./OPERATIONS.md](./OPERATIONS.md)) you need to deploy a public certificate from an accepted Internet Certification Authority (CA).

Some configuration example:
```
[default.tls]
certs = "/home/appservice/cert.pem"
key = "/home/appservice/key.pem"
prefer_server_cipher_order = true
ciphers = [
    "TLS_CHACHA20_POLY1305_SHA256",
    "TLS_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
]
```

## Database
The database can have some specific configuration. The Rocket project provides some [simple example](https://github.com/rwf2/Rocket/blob/master/examples/databases/Rocket.toml).

This is for SQLite using SQLx a rather simple one:
```
[default.databases.warehouse]
url = "./db/sqlite/warehouse.sqlite"
```

# Application
As mentioned we allow some appplication-specific configuration. See [../Rocket.toml](../Rocket.toml) for an example.

Each custom application configuration is a subsection of the configuration to make it easier to read and avoid confusion.

# Location of static files (frontend)
The frontend is a set of static files (e.g. HTML, Javascript, Cascanding Style Sheets (CSS)). You can configure where they are located. We recommed to put them in a subfolder "./static" of the web application.

You can configure this in [../Rocket.toml](../Rocket.toml).

An example:
```
[default.app.fileserver]
location = "./static"
```

# Security
## HTTP Security Headers
HTTP Security Headers are an additional line of defense to enable specific protection mechanisms against attacks (e.g. cross-site scripting) in the browser of the user.

You can customize some of the security headers of the application. Note: We do not make all headers customizable (e.g. CSFR) as they should be always on.

Additionally: If you configure a Content-Security-Policy (CSP) with "script-src" and "style-src" as below then the [application adds for web frameworks](../src/httpfirewall/securityhttpheaders.rs) - like [Svelte](https://svelte.dev/) - additionally a nonce and updates the [index.html](../../frontend/src/app.html) dynamically each request with the securely random nonce by inserting in the configured tags. In this way you do not need to specify unsafe-inline.

You can configure this as follows:
```
[default.app.httpheaders]
...
content_security_policy_inject_nonce_paths = ["^/$","^/index.html$","^/ui/*"]
content_security_policy_nonce_headers = ["script-src"]
content_security_policy_inject_nonce_tags = ["script"]
...
```
The following items are relevant for the configuration
* content_security_policy_inject_nonce_paths: This defines for which requests the nonce should be injected. We configure /, /index.html and the frontend-specific routes /ui/*
content_security_policy_inject_nonce_tags: This defines for responses to the Rocket routes (see previous item) in which tags the nonce should be added (here we say to the script tag). It makes only sense for script or style tags.
* content_security_policy_nonce_headers: This defines for the Content-Security Policy HTTP header which parts should include the nonce. It makes only sense to include it for script-src or style-src


Example:
```

[default.app.httpheaders]
content_security_policy = "default-src 'none'; base-uri 'self'; script-src 'self'; style-src 'self'; img-src 'self'; connect-src 'self'; font-src 'self'; object-src 'none'; media-src 'none'; child-src 'self'; form-action 'self'; frame-ancestors 'none'; upgrade-insecure-requests"
content_security_policy_inject_nonce_paths = ["^/$","^/index.html$","^/ui/*"]
content_security_policy_nonce_headers = ["script-src"]
content_security_policy_inject_nonce_tags = ["script"]
permission_policy = "accelerometer=(), ambient-light-sensor=(), autoplay=(), battery=(), camera=(), cross-origin-isolated=(), display-capture=(), document-domain=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), fullscreen=(), geolocation=(), gyroscope=(), keyboard-map=(), magnetometer=(), microphone=(), midi=(), navigation-override=(), payment=(), picture-in-picture=(), publickey-credentials-get=(), screen-wake-lock=(), sync-xhr=(), usb=(), web-share=(), xr-spatial-tracking=()"
referrer_policy = "no-referrer"
cross_origin_embedder_policy = "require-corp; report-to=\"default\""
cross_origin_opener_policy = "same-origin; report-to=\"default\""
cross_origin_resource_policy = "same-origin"
```
Here we set the HTTP security headers:
* [Permission Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Permissions-Policy)
  * Find here an online generator for permission policies: https://www.permissionspolicy.com/
* [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
* [ReferrerPolicy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy)
* [Cross Origin Embedder Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy) (COEP)
* [Cross Origin Opener Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy) (COOP)
* [Cross Origin Resource Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cross-Origin_Resource_Policy) (CORP)

You should consult the documentation of the headers, especially if you need to load resources (e.g. images) from another origin etc.

## OIDC

We provide [here](./EXAMPLE-CODEBERG-OIDC.md) an example using the free IdP Codeberg.org

Note depending on your application or IdP you may want to add additional configuration options and functionality of the openidconnect crate.

You find often configuration options based on the discovery endpoint of your IdP, which you can find under http://<issuer-url>/.well-known/openid-configuration.

Example for Codeberg.org: https://codeberg.org/.well-known/openid-configuration

See also the [OIDC RFC](https://openid.net/specs/openid-connect-core-1_0.html#Terminology).

### Authentication
You need to configure the IdP which allows to authenticate your user. Any IDP supporting OIDC and the Authorization Code flow is supported.

You need to specify for authentication the following items:
* issuer_url: This is the IdP issuing claims about the user to your application
* redirect_url: This is the url to which the IdP should redirect after successful authentication. You most likely need to configure this additionally in your IdP. This is based on the OIDC routes that are provided in this application (see [../src/oidc/routes.rs](../src/oidc/routes.rs)), ie it is https://<application-url>/oidc/redirect
* client_id: This is provided by your IdP for your application. Keep it confidential and do NOT commit it to your source code repository.
* client_secret: This is provided by your IdP for your application. Keep it confidential and do NOT commit it to your source code repository.
* scopes: scopes requested by your application

Example:
```
[default.app.oidc]
issuer_url = "https://codeberg.org/"
redirect_url = "http://localhost:8000/oidc/redirect"
client_id = "<CLIENT_ID>"
client_secret = "<CLIENT_SECRET>"
scopes = ["oidc", "profile", "groups", "email"]
```

### Authorization
Authorisation maps claims from the OIDC IdToken or OIDC UserInfo endpoint to roles. You can access them via user.mapped_roles and make decisions if the user should be authorised to access a specific route of your application.

You can map the claims using the following configuration items:

* roles_idtoken_claims: A list of strings that contain the claim names in the IdToken that contain roles 
* roles_userinfoendpoint_claims: A list of strings that contain the claim names in the UserInfo endpoint that contain roles 
* claims_separator: This is only needed if your IdP configures a string (and not a list) with multiple roles. You can configure how the roles in one string are separated (e.g. for "role1,role2,role3" the separator would be ",") Otherwise configure it as empty map ({}). You can configure for each claim a different separator


Example
```
roles_idtoken_claims = []
roles_userinfoendpoint_claims = ["groups"]
claims_separator =  { "groups" = ","}
```

We assume in this example that the roles of the user can be obtained from the UserInfo endpoint and they can be found under the claim "groups". This can be a string or a list. If it is a single string then they are separated by the ",". 

Your IdP can provide you information what claims it configured for the IdToken and the UserInfo endpoint.

