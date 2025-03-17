# Introduction

We describe here how you can configure with the example application an IdP that supports the OIDC Authorization Code Flow. We use here Codeberg.org (https://codeberg.org), because it is free and open source. You will need to create an own user account on Codeberg.org (Simply click on Register on the top right). Any user of your app would need to have also a Codeberg.org account (of course if you have a different IdP then they need to have an account with a different IdP).

However, you can use any IdP or platform that supports OIDC. Please look into their indivdual documentation as registering an application in the IdP differs, but in the background all use OIDC

# Registering a new application with Codeberg.org

After logging into Codeberg.org you need to register a new application. Click on your user profile on the top right and select "Settings".

![Codeberg.org user settings](./img/rust_oidc_usersettings.png).

Afterwards click on the left on "Applications". Scroll a bit down to the section "Manage OAuth2 applications" and enter the information for the application.

![Add a new application](./img/rust_oidc_codeberg_configapp.png).

We enter in this example:

Application name: Test OIDC

Redirect URIs: http://localhost:8000/oidc/redirect


The application name can be anything.

The redirect URI is the URL of your application under which it receives from the OIDC IdP the OIDC code to request tokens etc. We define in this example a localhost URL, which is the URL of the application if you use ```cargo make run``` to start it.

If you deploy your application to production (see also [OPERATIONS.md](./OPERATIONS.md)) it has a dedicated hostname and thus a dedicated URL, which you need to configure.

Afterwards click on "Create Application".

![Created application and client id/client secret](./img/rust_oidc_codeberg_appcreated.png).

Codeberg.org will tell you the client id and client secret for the application (we redacted them here, because you should keep them confidential and share with no one!).

They need to be configured for the application (see [CONFIGURE.md](./CONFIGURE.md)).

# Try it out

After you have configured client id and client secret you can run the application as described in ([BUILD.md](./BUILD.md)).

Navigate to http://localhost:8000

You will be redirected to Codeberg.org and asked to login with your Codeberg.org credentials (if you are not logged in already).

Aferwards you as a user need to approve that the application has access to your user information (Otherwise anyone could get your information!):

![Approve access to your user information by the application](./img/rust_oidc_firstlogin.png).

You can revoke at any time this access by going to Settings/Applications/Revoke access.

If you have authorized the application then you will see the application running.

![Application after authentication](./img/rust_oidc_app.png).

If you want to know what the application reads about you from Codeberg.org then go to 

http://localhost:8000/oidc/userinfo

![OIDC User information](./img/rust_oidc_userinformation.png).

We redacted some sensitive user information.

You will only a subset of the information accessible from codeberg.org, because we configured to request only a subset. However, you can change this by changing the configuration (see [./CONFIGURE.md](./CONFIGURE.md)) and/or the implementation (see [../src/oidc/](../src/oidc/)).

You will see in the default configuration mapped_roles only if you are member of an [organization in Codeberg](https://docs.codeberg.org/collaborating/create-organization/). For example, you can create an own organization or someone else adds you to an existing organization. 

Please note that every IdP can have different claims, e.g. Codeberg.org provides the roles in the claim "groups", but other IdPs may use different claim names.

# Deleting the application on Codeberg.org

If you currently do not develop or have no application deployed then it is strongly recommended to delete it from Codeberg.org to avoid abuse.

Go to the page where you created the application (Settings/Applications) and select "Remove" for the corresponding application.


# Some further information about Codeberg.org OIDC Support

You can find under the default OIDC discovery endpoint all OIDC options that codeberg.org supports:

https://codeberg.org/.well-known/openid-configuration

Please check also the Forgejo documentation (Codeberg.org is a Forgejo deployment) on OAuth2 provider:
https://codename.codeberg.page/@main/docs/latest/user/oauth2-provider
