# Introduction
We give here only a small excerpt of the architecture of the backend of the application. You are encouraged to read also the related documentation:
* [Rocket](https://rocket.rs/guide/) - web framework used

We implement the backend using Rust. This means it can be widely deployed on different devices and the cloud. For example, you can expose a web frontend of a software-defined radio (SDR) device or you can provide a highly scalable cloud web application.

# Rust
We use the [Rust Edition](https://doc.rust-lang.org/edition-guide/editions/index.html) 2024. Rust editions allow you to manage updates to the compiler and its infrastructure gracefully without breaking the application code.

# Rocket
We use as a web application framework [Rocket](https://rocket.rs/guide/) with the latest version. Rocket is a lightweight and powerful framework.

We use Rocket with [async requests](https://rocket.rs/guide/overview/#async-routes), which are suitable for high-performance applications

It has not an extensive ecosystem as, for example, Spring. However, it can extended in a very flexible manner using
* [Fairings](https://rocket.rs/guide/fairings/#fairings): intercept and rewrite requests/responses (with certain limitations)
* [Request Guards](https://rocket.rs/guide/requests/#request-guards): Intercept requests and decide to allow or deny them. This can be used for authentication/authorisation and also for preventing malicious requests.
* [Sentinels](https://api.rocket.rs/rocket/trait.Sentinel): A line of defense to abort starting the application if routes are added who have not implemented a sentinel

We use in this example Fairings (e.g. to add HTTP security headers) and Requests Guards (e.g. to implement OIDC authentication)

# Build
We use as a build tool [cargo-make](https://github.com/sagiegurari/cargo-make) as it has more advanced features compared to vanilla cargo.

The build script can be thus found in [Makefile.toml](../Makefile.toml). Of course cargo artifacts, such as [Cargo.toml](../Cargo.toml) are still used.


# Configuration
Rocket allows flexible configuration via [Rocket.toml](https://rocket.rs/guide/configuration/). This is mainly for configuring Rocket itself.

The application configures Rocket to also read an app-specific configuration file, where it [reads custom values](https://rocket.rs/guide/configuration/#extracting-values) for the application itself. See [CONFIGURE.md](./CONFIGURE.md) for the configuration.

You can find in [../src/main.rs](../src/main.rs) the code that extract the configuration using Figment. The module [](../src/configuration/) contains the structs and functionality to process this configuration.


See also [CONFIGURE.md](./CONFIGURE.md)

# Single Page Application
The frontend is a Single Page Application (SPA) written using SvelteKit. In Svelte (and also in many other frameworks) the concept of routes exists, i.e. the URL is changed to redirect the user at the frontend to different frontend views (components). This leads to a situation, where the backend interprets those as requests to the backend, which is incorrect.

Hence, we needed to configure a special Rocket route that makes sure that routes to frontend components are only redirected to the frontend.

You can find it in [../src/routes/redirect_frontend.rs](../src/routes/redirect_frontend.rs). All routes under /ui/* are considered frontend routes that should not be handled at server-side and thus are redirected to the frontend application.

# Static file serving (Frontend)
The frontend is a set of static files (e.g. HTML, Javascript, Cascading Style Sheets (CSS)). These are simply served when requested via the user's browser.

Rocket provides an excellent functionality for this ([FileServer](https://api.rocket.rs/master/rocket/fs/struct.FileServer)), but we do not use it in this application as it does not allow to configure authentication/authorisation for the static files.

Until this is the case, we have a simple custom static file server that requires authentication (see [](../src/routes/static_serve.rs)).

# Database
We currently use [SQLx](https://github.com/launchbadge/sqlx) as an async database backend for our Rocket application. The database is [SQLlite](https://www.sqlite.org/). SQLite is not suitable for production web applications, but for embedded devices or local development purposes. If you want to use a production database then use the SQLx backend for Postgres or MySQL.

SQLx allows to easily integrate queries into your database and return the results to the users. 

You can add the query to your endpoint as follows (see [](../src/routes/inventory.rs)):
```
#[get("/inventory")]
pub async fn inventory_handler(
    mut db: Connection<crate::database::Db>,
    user: OidcUser,
) -> crate::database::Result<Json<Vec<inventory::product::Product>>> {
    event!(Level::DEBUG, "inventory handler called");
    let products = sqlx::query!("SELECT id,name,price FROM product")
        .fetch(&mut **db)
        .map_ok(|product| Product {
            id: Uuid::parse_str(product.id.unwrap().as_str()).unwrap(),
            name: sanitization::clean_all_html(product.name.as_str()),
            price: Decimal::from_str(product.price.unwrap().as_str()).unwrap(),
        })
        .try_collect::<Vec<_>>()
        .await?;
    Ok(Json(products))
}
```

Note: SQLx requires you for every new query that you add to validate it (see also [./BUILD.md](./BUILD.md)). You can do this via
```
cargo make sqlx-prepare
```

Async database means we use [rocket_db_pools](https://api.rocket.rs/master/rocket_db_pools/). For SQLx it means databases such as Postgres, MySQL and SQLite are supported.

We plan to add to the example in the future an [Object-relational mapping](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping) (ORM) backend, such as Diesel. ORM is especially useful for complex business applications and logic.

This will be done once Diesel in Rocket has async support for SQLite.
# Logging
Logging is supported using [tracing](https://docs.rs/tracing/latest/tracing/). The main reason is that in async scenarios, such as we implement with Rocket requests, it is difficult to bring the messages in a consistent ordered view establishing temporality and causality when using "traditional" loggers, such as [log](https://docs.rs/log/latest/log/).

You can log events (moments in time) or time spans. Example:
```
use tracing::{Level, event};

event!(Level::DEBUG, "inventory handler called");
```

# Security
## General
Rocket has an emphasis on [security](https://rocket.rs/guide/introduction/#foreword).

However, Rocket also wants to keep its core clean and tested while any extension can be added as a plugin. Hence, we add several aspects in the example that deal with security.

## Input/Output Sanitization

### SQL injection attacks

You should carefully check the documentation of your database client on how to avoid SQL injection attacks.

For example, for SQLx you can find it here: https://sqlx.dev/article/SQLX_Security_Protecting_Your_Data_from_SQL_Injection_Attacks.html

### Cross-Site Scripting attacks
We provide input/output sanitization mechanisms based on [Ammonia](
https://github.com/rust-ammonia/ammonia). This sanitization removes malicious HTML/scripts from any text input from the user/output to the user. The idea is that you can sanitize any user input (e.g. new orders) before storing it in the database and to sanitize anything that comes from the database before returning it to the users. By doing so you can protect your users and your business from any harm coming from cross-site-scripting attacks (e.g. in our demo case malicious orders from customers or leaking of orders of other customers).

It is really important that you sanitize input coming from the user (e.g. via a form) and output to the user (e.g. from the database). Even if you have checked the input before adding it to the database you should AGAIN check it when returning it to the user. The reason is that there could have been a bug in the sanitization logic or new attacks are discovered that have not been previously sanitized.

We implement the sanitization logic in a simple module based on Ammonia. See [../src/services/sanitization.rs](../src/services/sanitization.rs). You can see how it is used, for example, in [../src/routes/inventory.rs](../src/routes/inventory.rs).

Additionally you always need to do input/output sanitization in backend AND frontend (see also [Frontend documentation](../../frontend/docs/ARCHITECTURE.md)).

## Form Validation
Additionally to input/output sanitization, one should use the mechanism provided by [Rocket for form validation](https://rocket.rs/guide/upgrading/#field-validation), if you use forms. This prevents some issues related to untrusted data from forms (all data from forms should be treated as untrusted - even if they are validated in the browser by Javascript).

## Limit request size
Another way of input/output sanitization is to limit the size what can be submitted in one request. Otherwise people will try to bring your application down by flooding it with large requests. 

This can be done by [configuring](https://rocket.rs/guide/configuration/#configuration) limits in Rocket for form submissions and JSON posts. You should also limit it for other type of requests.

If your application needs very large requests then you should not allow large limits, instead you should design your application that it splits large requests on the client side in smaller bits. For example, a 1 TB file can be split into 5 GB chunks and posted in 5 GB chunks to be afterwards reassembled on server side by a dedicated backend that is not the web application.

In this way, you can keep memory, cpu and other resource usage under control.

# Cross-site request forgery (CSRF) protection
Currently we do not include CSRF protection, which we will add in later releases to the example. It is strongly recommended to have CSRF protection for an application in production.

# HTTP security headers
HTTP security headers provide an additional line of defense against encryption-in-transit downgrade attacks and injection attacks (e.g. via [Content-Security Policy](https://en.wikipedia.org/wiki/Content_Security_Policy)).

We add HTTP Security Headers via a custom fairing: [../src/httpfirewall/securityhttpheaders.rs](../src/httpfirewall/securityhttpheaders.rs).

While most of them are normal headers, we have some more complex support to enable a Content-Security Policy without using unsafe-* statements. This requires that inline scripts (which are currently used by Svelte) have a randomly generated nonce for each request are inserted for the script tags.

They are [configurable](./CONFIGURE.md) in the application configuration.

# Authentication and Authorisation: OIDC
Rocket does not have any integrated authentication/authorisation framework, but one can add one using Request Guards.

We provide in this example how to integrate [OpenID Connect](https://en.wikipedia.org/wiki/OpenID) (OIDC) into the application with a request guard and additional routes as endpoints for the OIDC protocool (see [../src/oidc/](../src/oidc/)). This means you can plug in an external identity provider (e.g. a public one or the private one for your organisation).

We show here how to use the OIDC [Authorization Code Flow](https://openid.net/specs/openid-connect-core-1_0.html#CodeFlowAuth) suitable for web applications with a backend as in this example project. Furthermore, we implement also logic how to extract authorisation information, such as user roles, and make them available to the application endpoints. We use [openidconnect.rs](https://docs.rs/openidconnect/latest/openidconnect/) as a OIDC library and it supports also other OIDC flows which you can easily integrate into the application.

Additionally, we provide an [example how to integrate it with a public free IdP Codeberg.org](./EXAMPLE-CODEBERG-OIDC.md).

By default the whole application requires you to authenticate via OIDC (see [../src/main.rs](../src/main.rs)).

Keep in mind that if you add further routes to the application you need to integrate into the route the OidcUser to ensure it is protected (see for example [](../src/routes/inventory.rs)):

```
#[get("/inventory")]
pub async fn inventory_handler(
    mut db: Connection<crate::database::Db>,
    user: OidcUser,
) -> crate::database::Result<Json<Vec<inventory::product::Product>>> {
    ....
```