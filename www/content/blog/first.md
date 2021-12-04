+++
title = "How to choose the best typography for your blog"
date = 2019-11-27
+++
The vast majority of software has secrets. While this makes that software sound mysterious, like a femme fatale in a noir film, most software secrets are mundane. But that doesn’t mean those secrets aren’t important! Software secrets management can often be the difference between someone leaking confidential customer data or keeping that data secure. Understanding what secrets management is, and how you should approach it in your application, is a critical step in application maturity. In this post, we’re going to talk about what software secrets are, how you shouldn’t secure them, and what functionality you want in your secrets management approach.

What Are Software Secrets?
As we said, secrets are often mundane. Even basic static websites need to keep secrets. For most applications, secrets take the form of database passwords and encryption private keys. Even for static sites, secrets can take the form of keys used to connect to deployment systems or passwords for the web hosting server. As a developer, it’s your responsibility to protect these secrets. If someone malicious gained access to your application secrets, they’d gain access to whatever those secrets protect. Like we already mentioned, this might mean leaking your database data. If the secret is instead the password to a hosting server, they could replace your website with malware or deface it to spread a message you don’t support.

Laptop

Database Passwords
Perhaps the most common form of software secret is a database password. This is no different from any other password. Connecting your software to a database involves opening a network tunnel to a specific server, then providing a username and password. Often, the user authenticated by this password possesses elevated privileges compared to the standard user. Keeping this username and password confidential is critical for your application security. If this password were compromised, malicious actors could read everything in your database. What’s more, because of the elevated permissions this account possesses, they’d be able to write bad data or delete all your data too.

API Keys
Today’s applications rarely work on a simple client-server relationship. Often, your application connects to external services and leverages those to perform specialized actions. This access is managed by security keys issued by the remote API that services your application. These keys authenticate and authorize your application. If a malicious user gained access to one of these keys, they could connect to the API and impersonate you. They’d have all the same permissions you do on that remote system. This would allow them to read data that didn’t belong to them and perhaps to write fake data or delete data. As with your database, you’d have no log of who the malicious attacker was. They’d appear to be your application itself.

Deployment Keys
The world of software hosting cloud services is a ten-digit industry. It’s as likely as not that if you’re launching software today, you’ll be leveraging some sort of cloud hosting. Deploying software from your local installation to the cloud depends on secrets management, and those secrets are crucial. Someone who obtained your deployment key would have the power to overwrite the software deployed on your cloud platform. Often, deployment keys are capable of creating and destroying backups too. What’s even worse than someone deleting all your cloud software is someone subtly defacing your website. It’s horrible to know your application is offline. It’s much worse to find out it quietly served malware to your customers for six months and you never knew.

Forms of Secrets Management
Encryption Keys

There are a variety of ways to manage your software secrets. Each has its own shortcomings, and it’s up to you as a developer to find the one that works best for your environment. Let’s walk through some of the different options and talk about the ways they do and don’t work. By understanding the potential options, you can make the best decision about secrets management for your application.

Hardcode Secrets
This is only secrets management in the loosest form of the term. In this version of secrets management, you hardcode your application secrets. This is a bad idea, just like it’s a bad idea to reuse any kind of password. Given that it’s universally acknowledged to be a bad idea, why do so many applications use it? Well, for starters, it’s a lot easier for the application developer. You just drop the password right in the code! You’re not really managing anything at all.

The shortcomings of this approach are numerous. Sure, if you’re the only developer on your project and you always keep your source code completely secure, hard-coding secrets won’t likely carry too many repercussions. But hard-coded passwords mean that you’ll be reusing your passwords across different environments. Your testing and QA and production environments will all use the same password. As a result, your most important data will be no more secure than your throwaway testing data. What’s more, if you expand your team, you’ll have no way to differentiate user access. Who logged in to the database and made that change? Well, they used the one password that’s hard-coded. You have no way of knowing.

Secret Encryption
As teams mature, they recognize that they can’t leave plaintext passwords in their code. This is a good step! Often, they’ll turn to a system like git-crypt to selectively encrypt secrets within their application. Instead of leaving their secrets in plaintext, they’ll encrypt the files where secrets live. This has the benefit of protecting those secrets in the event someone gains access to their source code. Often, teams take this step as they start to expand the software team but still employ a single-digit number of developers.

While this approach is more secure than hardcoding secrets in plaintext, it adds overhead to the developer workflow. All developers have to have knowledge about gpg and have to manually synchronize and import their colleague’s public keys, so that changes to secrets are encrypted for everyone. This process has to be repeated for each repository containing secrets and each developer working on it. This solution doesn’t scale well.

Environment Variables
After embarking on a mission to encrypt their software secrets, most teams continue to grow. At some point, they recognize that they can’t continue to use the same password for all their environments. Sometimes, they realize this organically. A developer makes a case for diversifying passwords due to the risks of password reuse. But other times, they learn this lesson the hard way. Someone accidentally logs on to the production database and tweaks data while they thought they were operating on the test system. Or they leak customer data due to a malicious intruder.

No matter the reason, the next step most teams take is to store their application secrets in environment variables. To be clear, this is a huge step forward in terms of security. Now, not every developer has access to every secret in the system. Instead, they have access to credentials used in lower risk environments (for example, a secret for a dev environment instead of the production environment). The secrets for production environments are knowledge limited to far fewer employees. Instead of storing the secrets right inside the application, they’re stored on the system where the application runs. Then, the software interfaces with the underlying operating system and reads configuration values at system start-up.

While this is a major benefit to application security, it’s not a panacea. Often, teams using environment variables reuse the same application secrets for every developer. Because it’s usually challenging to change environment variables on multiple systems, teams that utilize environment variables rarely rotate their application secrets. While password rotation is considered harmful in end-user scenarios, it’s still a benefit at the application secret level. What’s more, environment variable access isn’t logged or locked down. Anyone who can access your production server (for instance, through an SSH tunnel) is capable of reading environment variables on the running server.

Secrets Management Software
For mature, security-conscious organizations, secrets management software is the end game. Instead of manually setting and storing secrets within the application, they hand their secrets off to a security-focused application. That application stores those secrets. Then, the user’s software application contacts the secrets manager when it starts up. The secrets manager sends secrets back to the application, and the application utilizes them through its runtime. As a result, application secrets aren’t stored in a shared location. Moreover, the ease of modifying secret values within the application means it’s much easier to rotate those secrets. And, because the secret manager application is a full software stack, teams will often generate and assign specific secrets to specific developers. When the developer needs access, they log in to the portal, retrieve their specific secret, and access the system they need. Their actions are logged, and you can limit the control they have.

One thing to keep in mind when choosing a secrets management tool is that your software will depend on its uptime to function. When your application starts up, it needs to make a network request to an external service first, before it can connect to services like the database. If the network connection fails or that service is down, the application won’t start. That would be a big problem! This is why you shouldn’t just evaluate secrets management software on the variety of features provided, but also on the stability of the system. A system that’s highly available is often more valuable than a system with the absolute perfect feature set.

The Best Secrets Management Software
Encrypted Connections

The best secrets management software combines all the different abilities we’ve talked about. Not only is the software highly available and very stable, but it makes authorizing users for different systems easy and straightforward. It logs access when a user utilizes a secret and makes it easy to rotate secrets on a set pattern. When a user leaves the organization, it’s trivial to retire their secrets so that they can no longer access any data. It works across any programming language and operating system without needing complicated configuration. It’ll integrate with any cloud service provider or work for on-premise servers.

For all these reasons, SecretHub leads the pack of available secrets management software. It’s simple to add and integrate into any software system and works across cloud providers. What’s more, it boasts better than 99.99% uptime, so you can rest assured that whenever your application is starting up, SecretHub will be there for you.

As we’ve noted, many teams progress through different stages of secrets management. At each step of the way, they improve their security posture and reduce the risk to their application. But for many teams, that process involves a significant amount of time exposing their application to more risk than they need to. If your team is in that situation, you can skip the intermediate steps. SecretHub provides a free community edition that unlocks your team’s security without needing to worry about cost. And as your team grows, SecretHub grows with you. There’s no risk to trying out SecretHub, but there’s definitely a risk to continuing your current practices. What’s stopping you from trying SecretHub today?