# az_sovereign_keystore
Keystore solution that leverages azure keystore while ensuring sovereignty


With the current uncertainties in the world digital sovereignty is an important theme again, especially in Europe. We have a strong dependency on US hyperscalers like Azure, AWS, and Google. These hyperscalers provide a scale and a feature-set that can not be matched by any of the European alternatives now or in the near future. So our dependency on American hyperscalers will prevail for the next decade I would expect. So the question should be how to leverage the huge data-infrastructure of the large hyperscalers, while still safeguarding the access to our own data. All hyperscalers offer advanced features for data-protection which BYOK (= Bring Your Own Key), and HYOK (= Hold Your Own Key) or CYOK (= Control Your Own Key). All these systems provide additional data-security and the external security audits provide (some) assurance that these features deliver as promised. However, all these systems have two weak spots:

1. These systems are not fully open source, so it is not possible to review the workings.
2. The actual deployment of these systems and the underlying infrastructure is not managed by you.

Furthermore the more advanced data-encryption feature do impose new security risks, as your organisation needs to be ready to manage its own infrastructure and human procedures to implement a BYOK or HYOK, and if either that infrastructure, the or the human factor fails you might lose your primary key and thus access to all your data. 

In this repository I am going to study and show in a proof-of-concept (PoC) how you maintain your data-sovereignty, while simultaneously leveraging the strenghts of hyperscaler (Azure in the PoC) as much as possible via Open Source tooling. The product will be a API-service that you can deploy and use with your favorite tools and languages.

However, it will take a month or two to mature this repository, as this is a personal project which is unrelated to my daily job, which is need to keep live rolling. `;-)`


##TO BE CONTINUED shortly
....        
