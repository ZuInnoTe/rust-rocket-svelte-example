# Introduction
We cannot give a full-overview of all possible things to consider for operating an application. See this as an introduction to look for more.

# Environments
You should have different environments for your application so that you do not impact your users when developing new features of the application or when doing load testing. As a minimum set of environments you can think of:
* Local development - this is the environment used by the developers - often a local machine - to develop the application
* Development - this is the environment where multiple developer changes are brought together and deployed to see that they can work together
* Test - here developers and users do functional tests, integration tests, performance tests etc.
* Production - here the application is provided to the end users to support them in their business tasks

# Secrets Management
You should manage secrets (e.g. database passwords) in dedicated secret vaults and fetch them during runtime.

NEVER put secrets in container images or application packages - this is insecure.

Secret vaults often also allow rotating  (changing) regularly secrets for an improved security.

An example for a secret vault is [AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/) or [HashiCorp Vault](https://developer.hashicorp.com/vault/docs).

The best way would be though to avoid using secrets at all in your application. Cloud provider offer for this short-living tokens (e.g. AWS IAM Roles or Azure Managed Identities). Their advantage is that you do not need to manage secrets anymore at all.

Standards are emerging to enable this [cross-cloud](https://zuinnote.eu/blog/?p=2273).

# Security Hardening
You should harden your application and the environment where it runs. The [CIS Security Benchmarks](https://www.cisecurity.org/cis-benchmarks) provide some information how you can do this.

Additionally, for containers and infrastructure-as-code, you can use static analyis tools, such as [terrascan](https://github.com/tenable/terrascan) and/or [checkov](https://github.com/bridgecrewio/checkov) to get insights how to improve your environment.

# Upgrading/Patching
You should regularly patch/update third party dependencies of your application and the underlying infrastructure (e.g. container image and VMs). These patches should happen frequently (e.g. weekly) and they must be also deployable ad-hoc (e.g. in case of zero-day exploits).

One important thing one needs to consider is to have a very low (or non-existing) down-time of the application, e.g. by having load-balancer redirecting traffic to multiple instances of your application or only redirecting traffic to a newly deployed instance of your application if health-checks have been passed.

You should be able to restore infrastructures, configurations, binaries from a versioned repository so that you can easily recover your infrastructure in case of failures. Obviously you should test beforehand in another non-production environment that this works as expected.

# Backups/Restore
Usually applications process crucial data in databases, object stores, filesystems etc. Loosing data often means loosing lifes (e.g. in healthcare domain), money, jobs etc. 

Hence you should back the data up in a save place outside your application environment.

You should regularly test if the data can successfully restored using restore test where you also need to test if after a restore your application still works.

Important [indicators](https://en.wikipedia.org/wiki/IT_disaster_recovery) are:
* Recovery Point Objective (RPO)
* Recovery Time Objective (RTO)

# Logging
You should configure the logging properly and possibly transfer the logs to a central logging service to get a better overview across different application instances what is going on.

If you use log files then make sure 1) that they do not grow infinitely 2) that logs are kept only for a certain amount of days (e.g. 30) 3) that log files are rotated and compressed every day 4) are stored on a separated partiton to avoid that the operating system disk spaces runs out.

All these things can usually be done by configuring the log4j2 of the application (see [documentation](https://logging.apache.org/log4j/2.x/manual/configuration.html)) and no custom code or shell scripts are required for this.
# Monitoring
This depends on your environment and the existing monitoring application that you have. 

Within this monitoring application you should be able to configure rules that fire on errors appearing in logs and automatically triggering actions (e.g. on database error restart the database).

Of course also humans could be notified via e-mail.

For example, on AWS you can choose [AWS Fargate](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/AWS_Fargate.html) to run a container and it automatically integrates the logs into [AWS Cloudwatch](https://docs.aws.amazon.com/cloudwatch/). There you could use Cloudwatch Metric/Alerts to trigger on error automated actions. AWS Fargate can check the health endpoint automatically exposed by Spring Boot Actuator. If it detects it is unhealthy it will automatically request additional healthy container instances to serve end users requests.

Of course you can use any other monitoring stack to do essentially the same.

# Load balancer and web application firewall
You should configure a [load balancer](https://en.wikipedia.org/wiki/Load_balancing_(computing)) for high availability and meeting your users demands. This may also mean to have a cache deployed for the session management. There are various way on how you can configure a load balancer - from a self-managed one (e.g. [NGINX](https://nginx.org/), [Apache HTTPD](https://httpd.apache.org/docs/current/howto/reverse_proxy.html), [Traefik](https://traefik.io/)) to a cloud managed one (e.g. [AWS Application Load Balancer](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/introduction.html) or [Azure Application Gateway](https://learn.microsoft.com/en-us/azure/application-gateway/overview)). Usually the load balancer take also care that traffic is not redirected to unhealthy nodes (e.g. overwhelmed web application instances). You need to keep the software of the load balancer up-to-date to have the latest security fixes, support latest encryption algorithms and performance benefits.

Note: You should ALWAYS encrypt traffic (TLS) between backend and load balancer as well as between load balancer and browser. Nowadays it is NOT acceptable anymore to have anywhere unencrypted traffic (even in the own data centre).

Additionally you should configure a [web application firewall](https://en.wikipedia.org/wiki/Web_application_firewall) (WAF) (often you have this as an additional module to your load balancer). Keep the WAF up-to-date - this includes the software, but also the rules. The rules change continuously! You find an updated ruleset by OWASP [here](https://coreruleset.org/). Choose the right [paranoia level](https://coreruleset.org/docs/concepts/paranoia_levels/) of your rules. Usually paranoia level describe the trade-off between false positives and false negatives. The more your application needs protection (e.g. processing of sensitive or personal data) the higher the level should be.

If your application is exposed to the Internet then you should seek for mechanism for protecting against [Distributed Denial of Service](https://en.wikipedia.org/wiki/Denial-of-service_attack) (DDoS) attacks.