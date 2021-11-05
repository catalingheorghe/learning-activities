### Introduction to Azure Fundamentals

https://docs.microsoft.com/en-us/learn/modules/intro-to-azure-fundamentals/introduction

Azure - cloud computing platform (from web hosting, to virtual host for custom
software solutions; databases, AI services, IoT etc.)

#### Cloud Computing

 - computing services offered via the internet, from a datacenter
     - basic services: compute power and storage
 - pay as you go, and usually for what you use

Why?

 - fast paced delivery required today
 - features like voice recognition, ML ... expected (cloud services)
 
#### Azure

 - set of cloud services

 - flexible - e.g.: hybrid
 - customizable: build and deploy how you want
 - security
 
More than 100 services. First step is usually to move existing app in a VM in
the cloud, but Azure can do much more than that (dynamic storage services, AI
services, etc).

How does it work?

 - uses virtualization - a hypervisor abstracts resources in virtual machine

 - multiple datacenters around the world, with servers. Each server can run
   multiple VMs. A network switch provides connectivity to the servers in a 
   rack. 

 - one server in a rack runs a "Fabric Controller" - which connects to an
   "Orchestrator" - this is where the user requests come in (via an API)

Azure portal

 - web-based unified console - can manage everything, create dashboards
 - designed for continuous availability (has a presence in every datacenter)
 
Azure Marketplace

 - apps and services certified to run on Azure (from Microsoft or others)

#### Tour of Azure services

10 main categories

 - compute (virtual machines, app services ...)
 - networking (connection in cloud and with on premise, vpn, load balance ...)
 - storage (disk, file, blob, archival ...)
 - mobile (develop apps, send notifications ...)
 - databases (proprietary and oss engines)
 - web (web applications, publish api, ...)
 - IoT (creat and monitor IoT assets)
 - big data (open source cluster services, analytics)
 - AI (forecast future behaviour from existing data; ML models mostly;
   cognitive services - image, speech recognition, NLP, bing search)
 - devops (automate software delivery, buiild and release pipelines)
 
https://docs.microsoft.com/en-us/learn/azure-fundamentals/intro-to-azure-fundamentals/media/azure-services-6c41a736.png

#### Azure accounts

 - when creating an account, a subscription is created; but multiple subs can
   then be added
   - account <- subscription(s) <- resources
   - a free account - free access to some services for 12 months, a credit
     https://azure.microsoft.com/en-us/free/free-account-faq/
 - Learn sandbox - temporary subscription added to your account, that will be
   cleaned up at the end

### Discuss Azure Fundamental concepts

https://docs.microsoft.com/en-us/learn/modules/fundamental-azure-concepts/introduction

Tailwind Traders - home improvement retail business, now on-premise looking
to take advantage of the cloud.

#### Different types of cloud models

3 deployment models for cloud computing

 - public cloud
   - services offered over public Internet; cloud resources owned by third-party
   - no initial scale-up cost, pay as you go
 - private clound
   - computing resources used by only one organization; either on-site in that
     organization's datacenter, or hosted by a third-party service provider
   - hardware must be purchased, maintanence must be done; complete control over
     resources and security
 - hybrid
   - combination of the above, data and applications can be shared between them
   - org can decide where to run apps, control over security, legat etc
   
#### Cloud benefits and considerations

Cloud computing advantages

 - high-availability - depending on the SLA chosen
 - scalability
   - vertical - more ram and cpu in a node
   - horizontal - more instances of resources
 - elasticitly
   - autoscaling, on-demand, for example
 - agility - deploy resources matching requirements
 - geo-distribution
 - disaster recovery - data safety
 
Expenses

 - capital  - capex - upfront money spent on infrastructure
 - operational - opex - money on services or products spent now
 
Consumption based model - pay for what you use

#### Cloud services models (PaaS, IaaS, SaaS ...)

Service models that define the shared responsibilty between a coloud provider 
and cloud tenant.

IaaS - Infrastructure as a Service
 - closest to managing physical servers; cloud provide will only keep hw updated
 - operating system, network config, up to you
 - in azure: virtual machines as new compute devices
 
PaaS - Platform as a Service
 - managed hosting environment; provider manages the VMs and networking
 - you deploy the applications into the managed environment
 - in azure: Azure App Services - can upload apps w/o worrying where they run
 
SaaS - Sofware as a Service
 - manages all aspects of app enviroment - from VMs to data storage and apps
 - you only provide the data
 - e.g.: Office365 - you only create the content inside it

https://docs.microsoft.com/en-us/learn/azure-fundamentals/fundamental-azure-concepts/media/shared-responsibility-76efbc1e.png

Serverless computing - this is like Paas - the app runs in the cloud, you as a
developer don't have to care where. There are still servers behind, but all is
provisioned by the cloud infrastructure.

### Describe core Azure architectural components

https://docs.microsoft.com/en-us/learn/modules/azure-architecture-fundamentals/introduction

#### Overview of subscriptions, management groups, resources

Resource organization level

 1. management groups - access, policy and compliace for multiple subscriptions
 2. subscriptions - mapping of user accounts and resource groups 
 3. resource groups - logical containers into which web apps, databases, etc
    are deployed
 4. resources - instances of services, like VMs
 
#### Regions, availability zones and region pairs

Resources are created in regions - geographical locations that have datacenters.
One ore more datacenters can be used.

Datacenters are not exposed to users, only regions.

Region
 - geographical area with one ore more tightly connected datacenters
 - some are special regions with tight access and compliance (US DoD, China)

https://docs.microsoft.com/en-us/learn/azure-fundamentals/azure-architecture-fundamentals/media/regions-small-be724495.png

Availability zones
 - physically separated datacenters with indepenent infrastructure
 - this can offer high availability (via redundancy) to your app
 - not all regions offer availability zones
   https://docs.microsoft.com/en-us/azure/availability-zones/az-region

Region pairs
 - pairing of regions so that there can be a fail-over in case of major events
   e.g.: US West with US East
 - these can be used for providing reliable services and data redundancy 

#### Azure resources and Resource Manager

Resource
 - manageble item (VMs, storage accounts, web apps, virtual networks...)

Resource group
 - container with related resources from your point of view
 - all resources must be in a resource group, and can be in a single one
 - can't be nested
 - if deleted, all resources inside are deleted
 - RBAC permissions can be applied per resource group
 
Azure Resource Manager
 - this is the service that manages resources
 - any Azure tools, API, SDK request ends up here
   https://docs.microsoft.com/en-us/learn/azure-fundamentals/azure-architecture-fundamentals/media/consistent-management-layer-feef9259.png
 - allows management via declarative templates - JSON files
 - can define dependencies so deployment of resources is in the correct order
 - tags for further logical grouping; can see billing per tags for example
 
#### Azure subscriptions and management groups

Subscription
 - linked to an Azure account (Azure AD, or other directory it recognizes)
 - an account can have multiple subscriptions
 - each has its one billing reports and access-management policies
 - a single subscription has some limits - eg max Azure ExpressRoute circuits

Example of separate subscriptions
 - different environments - dev, testing, security
 - mapped to organizational structure
 - production vs r&d for billing

Azure management groups
 - are containers of subscriptions and/or other management groups
 - allow specifying polcies at scale
 - max 10,000 groups in a directory, max 6 levels deep
 
#### Exercise - website hosted in Azure

WordPress website - Azure App Service instance

 - App Service - HTTP based services for building and deployin web-based apps
   without setting up any infrastructure
 - Azure Marketplace - applications certified to run on Azure (ie WordPress)
 - Resource Group - first thing to create to group all that will be used
   (in the learn sandbox, already created)
   
Careful when selecting the App Service plan - this will give the pricing tier.

Once "deployed", you can go to the App Service from Resources. It will have a
FQDN that you can acess to setup the WordPress site.
 
 


















   
 

 




