## Azure Core Services

IaaS, PaaS, Networking, Storage

### Explore Azure compute services

https://docs.microsoft.com/en-us/learn/modules/azure-compute-fundamentals/introduction

#### Overview of Azure compute services

 - Virtual Machines - allows customization of OS and software (IaaS)
 - Virtual Machine scale sets - set of identical VMs
 - Container Instances and Azure Kubernetes Services - lightweight virtualized
   app environments (scaled out, started and stopped quickly)
 - Azure App Services - web, mobile and API apps (Paas)
 - Functions - care only about the code running your service, usually short
   request processing, can be from another Azure service (often REST request)
   
#### When to use Azure Virtual Machines?

 - total control over software; no need to buy the physical server (IaaS)
 - predefined templates - VM images - OS, often dev tools

"lift and shift" - image your physical servers, move them to cloud VMs

Scale Sets
 - scale sets - load balanced set of identical VMs (no need to set that load
   balancing up manually)
 - number of VMs can automatically adapt, on demand or by a schedule
 
Azure Batch
 - large scale parallel and high performance computing (HPC)
 - pool of VMs, queues tasks as jobs, requeues work
 
#### When to use Azure App Services?

Build and deploy web apps, backgrou jobs, REST APIs w/o infra management (Paas)
 - can do automatic deployment from github, Azure DevOps or any git repo
   
Costs
 - you have an App Service Plan and then you pay for the compute resources used
 - the plan determines how much hardware you can use
 
Types of App Services
 - web apps - windows/Linux as host, apps in asp, java, ruby, node, php, python
 - API apps - build a REST API
 - WebJobs - run a program or script in the same context as a web/api/mobile app
   - i.e.: schedul and run background tasks as part of your app logic
 - Mobile - build a back end for IOS and Android apps
   - store mobile data - SQL, authenticate via thirdparty, push notifications,
     execute back-end logic C# or Node.js
     
#### When to use Azure Container Instances or Kubernetes Service?

Multiple instances of an app on a host -> containers.

No need to manage the underlying OS. Designed to be created, scaled out, stopped
dynamically. Docker as container engine is supported.
Containerized app - bundle of the app and its dependencies.
Can be more easily orchestrated.

Management of both Docker and Microsoft based containers in Azure
 - Azure Container Instances
    - allows you to upload your container and it runs them (PaaS)
 - Azure Kubernetes Service (AKS)
    - automating, managing large number of containers - orchestration
    
Kubernetes
 - pods (1 or more containers) on cluster nodes
 - moving pods between clusters, restarting them
 - scale manually or automatically
 - staggered update, auto rollback
 - persistent volumes as shared storage, or use of cloud storage
 - network isolation, grouping, exposing to outside
 - offers extension capabilities to certain events
 
Containers in solutions
 - usually micoservice architecture (small, indepent pieces)
   - basic eg - website: 3 containers - front end, back end, storage

Microservices
 - small, loosely coupled web service; self contained business capability
 - its own codebase
 - loosely coupled architecture
 - should hotd its own data and state - autonomous
 - well define APIs are mandatory
 
#### When to use Azure Functions?

Serveless computing - abstraction of servers, only code execution; great for
event driven scenarious - platform schedules the function (code) to run and
can scale the number of compute resources based on event rate; connecting to
other services is specified in metadata via declarative bindings; allows for
micro-billing (order of minutes of computing time)

Azure Functions
 - quick (seconds or less) work, event driven; automatic on-demand scaling
 - stateless - default; stateful - a context is kept (Durable Functions)

Azure Logic Apps
 - execute workflows, not code, built from predefined logic blocks
 - visual designed in Azure portal or Visual Studio; persisted as JSON
 - more that 200 connectors and processing blocks; can also build custom ones
 - stateful

Function and Logic Apps can be called from one another.

#### When to use Azure Virtual Desktop?

Cloud hosted version of Windows that can be used from anywhere.
Native app as a client or modern browser.

Dedicated desktops can be provided to team members - they can install and
manage apps on them.
 

### Explore Azure networking services

https://docs.microsoft.com/en-us/learn/modules/azure-networking-fundamentals/

Servers with customer data in one region, branch offices in others.
Keep on-premise data center, but offload peaks into VMs in the cloud, under the
same IP addressing scheme.

#### Azure Virtual Networking

Azure Virtual Networks (vnet)
 - communication between Azure resources (VMs, DBs ...), with Internet users
   and with on premise sites
 - isolation and segmentation - each vnet with its own IP address space
   (private or public); name resolution - internal or external DNS server
 - internet - to the internet, default; for incoming, can configure a public
   IP address or a public load balancer
 - between Azure resources
    - virtual networks (VMs, kubernetes ...)
    - service endpoints (connect to Azule Storage, SQL ...)
 - with on-premise resources
    - point-to-site VPN (classic user vpn into corporate network)
    - site-to-site VPN - link between your VPN gateway and Azure's
      (over public internet)
    - Azure ExpressRoute - for greater bandwidth and stricter security
      (dedicated private connection to Azure)
 - routing network traffic - by default, Azure routes between vnets, Internet,
   on premise. Control via Route Tables and BGP (propagate on-premise BGP
   routes to Azure network service)
 - filter network traffic - Network security groups (i.e.: filter on IPs),
   Network virtual appliances (like a hardened VM that does firewalling or WAN
   traffic optimization)

Connect Virtual Networks
 - Peering linking virtual networks together so resources in different regions
   can communicate.
 - UDR User Defined Routing allows network admins to configure routing between
   subnets in a VNet, VNets, on-premise.

#### Azure Virtual Network settings

Create and configure from Portal, Power Shell locally, or Cloud Shell.

Creating
 - network name - unique in subscription
 - address space - CIDR, unique within subscription and with connected nets
 - resource groutp - mandatory for any azure resource to be in on
 - sebnets within the vnet
 - basic or standard ddos (standard is a premium service)
 - service endpoints - cosmos db, service bus ...
 
Further settings
 - network security group - filterting traffic; created separately
 - route table - add custome route tables besides the default ones
 
After created
 - connected devices - connect machines to vnet
 - peerings - link vnets

#### Azure VPN Gateway Fundamentals

VPN Gateway - a type of virtual network gateway; they allow site-to-site,
point-to-site or network-to-network (between 2 vnets)

 - oly one VPN gateway per vitrual network
 - gw can be used to connect to multiple locations (other vnets, on premise)
 
Authentication: pre-shared key
Encryption: IKE, IPSec

Policy-based VPNs
 - statically specified set of IP addresses to encrypt
 - static routing
 - IKEv1 only
 
Route-based VPNs
 - IPsec tunnels treated like network interfaces, IP routing done over them
 - support IKEv2
 - supports dynamic routing
 
VPN gateway specifications
 - basic - only for dev/test; low bandwith, no BGP
 - 1,2,3 - tiers with 30 connections, max 1.5 Gbps, BGP

Deploying a VPN GW
required Azure resources
 - vnet
 - gateway subnet - must not overlap with the on-premise one you target
 - public IP address (for Basic, it's dynamic but won't change unless you
   delete and recreate the VPN gw)
 - local network gateway - details about the on-premise network - gateway
   public IP and routable networks
 - virtual network gateway - the gw, VPN gw or Express Route
 - connection - a logical connection between the last two itmes
 
High-availability

Active/Standby
 - VPN gw are deployed by default in pairs - on active, one standby, to cover
   maintenance windows or disruptions
 - connections are interrupt but then restored
 
Active/active
 - unique public IP addresses to each instance; separate tunnels
 - use BGP to adjust routes on failures
 
ExpressRoute failover
 - if there is a problem with the expressRoute (which is a dedicate physical
   connection), a VPN gw can act as a failover, using the internet

Zone-redundant gateways
 - availability zones, can be deployed in a zone-redundant config

#### Azure ExpressRoute fundamentals

Extend on-premise into Azure over a private connection

 - layer 3 connectivity between on premise and Microsoft cloud
 - built-in regundancy
 - connectivity uptime SLA
 - access to office 365, dynamics 356, azure vms, cosmos db, storage
 - connection between private sites that use different ExpressRoutes
 - BGP - route exchange between on-premse and Azure resources
 
Connection types
 - colocation - datacent at ISP, can request a virtual cross-connection L2, L3
 - peer to peer ethernet - datacenter directly to Microsoft L2, L3
 - any-to-any networks - connect to your WAN, like any other branch L3


### Expore Azure Storage services

https://docs.microsoft.com/en-us/learn/modules/azure-storage-fundamentals/introduction

note: Azure storage != Azure database services

#### Azure Storage account fundamentals

Services
 - blob, fileshares, disk storage, no-sql (key-value pairs), queue storage
   (message storing)
 - note: disk storage - virtual disk must be linked to a Virtual Machine
  
Tiers
 - hot, cold, archive (frequency of access)

Azure Storage account (poweshell, portal, cli)
 - required to hold data objects
 - an account is a unique namespace for your Azure Storage data

#### Disk storage fundamentals

 - disk for Virtual Machines
 - performance tiers - standard HDD, SSD to premium SSD

#### Blob storage fundamentals

 - large object data, unstructured, any format
 - accessible from anywhere, no disk management required
 
Ideal for:
 - serving images or documentes to a browser, files for distributed access, 
   streaming video and audio, backup and restore data, storing data for
   analysis, storing up to 8TB of data for virtual machines
   
Account -> Containers -> Blobs
 - containers organize your blobs

#### Azure files fundamentals

 - cloud managed file sharing - SMB, NFS (preview only)
 - can be mounted from cloud apps/vms, on-premise hosts etc
 - can be mounted across geographical locations

Security
 - encrypted at rest (Azure Files)
 - encrypted in transit (SMB)

Access
 - via an URL from anywhere in the world
 - Shared Access Signature (SAS) tokens allows asset to a private file for
   limited amount of time; SAS token part of URI

#### Blob access tiers

Organize your data by frequency of access and planned retention time.

Access tiers for blob storage
 - hot - frequent access (e.g.: images for your website)
 - cool - infrequent access and stored for at least 30 days (e.g.: invoices)
 - archive - rare access and stored for at least 180 days, different latency
   requirements (e.g.: backups)
   
Account level - only hot and cool tiers can be set
Blob level during upload or after upload - all tiers available


### Explore Azure database and analytics services

https://docs.microsoft.com/en-us/learn/modules/azure-database-fundamentals/introduction

globally distributed database service
relational, no-sql, in-memory


#### Explore Azure Cosmos DB

distributed data, that can be accessed through different APIs to match current
application - SQL, MongoDB, Cassandra, Tables, Gremlin => easier to migrate
different databse services

schema-less, constantly changing, "always on" data

low level storage format: atom-record-sequence (ARS), then one of the previous
APIS is created on top when creating the database

#### Explore Azure SQL Database

 - Microsoft SQL Database engine
 - no need to manage infrastructure
 
 - PaaS service, offers 99.99% availability, built-in high availability,
   backups
   
 - Azure Migration Service - easy migration of SQL Server, you just change
   the connection string in the apps

#### Create an SQL database

Azure portal -> create resource -> database -> sql
 - subscription, resource group
 - sql server instance - public endpoint, TLS 1.2, no sql defender
 - sample data
 - after resource deployed - Set server firewall and "yes" allow azure services
   and resources to access this server
 
Test database
 - my resources -> query editor (preview)
 - try to login with sqluser -> error will appear - get the client IP
 - overview - set server firewall - add client ip (start ip, end ip)
 - try to login again

#### Explore Azure database for MySQL

LAMP stack on premise
 - Azure App Service - Web Apps - built-in functionality for PHP web apps,
   on a Linux server running Apache
  - what about MySQL?
  
Azure MySQL
 - MySQL Community Edition 5.6, 5.7, 8.0
 - 99.99% availability
 - point in time restore as far bask as 35 days
 - scaling on demand
 - protect data at-rest and in-motion
 - migration: Azure Database Migration Service
 
Tiers
 - from lightweight (a few dollars a month for a small app) to heavy workloads
 - can auto-scale and pay as you go
 
#### Explore Azure database for PostgreSQL

Azure PostgreSQL
 - based on community version open-source version
 - similar to MySQL benefits

Two deployment options
 - Single Server
    - Pricing Tiers: Basic, General Purpose, Memory Optimized
 - Hyperscale (Citus)
    - horizontal scaling of queries across mulitple machines by using sharding

#### Explore Azure SQL Managed Instance

Similar to Azure SQL Database, but with some specific features
 - e.g.: Cyrillic characters for collation
 - https://docs.microsoft.com/en-us/azure/azure-sql/database/features-comparison/
 
Migration process - discover features in use, select clound service, adjust
settings to resolve issues, migrate, cutover, optimize
 - https://docs.microsoft.com/en-us/azure/azure-sql/migration-guides/managed-instance/sql-server-to-managed-instance-guide

#### Explore big data and analytics

"GPS tracking system for all of its delivery vehicles. The new system provides 
real-time tracking data to your primary datacenter. Your CTO wants your team to 
look at several years of tracking data in order to determine trends. For example, 
an important trend might be a spike in deliveries around the holidays that would
require hiring additional staff."

Big data - large volumes of data that are so large that traditional forms of
processing and analysis are not sufficient.

Azure Synapse Analytics
 - data warehousing, limitless, with querying

Azure HDInsight
 - open source analytics service - can run spark, hadoop, kafka, storm
 - ETL, data warehousing, iot

Azure Databricks
 - helps you build AI solutions by extracting insight from your data
 - python, scala, r, sql, java; rensorflow, pytorch, scikit-learn

Azure Data Lake Analytics
 - on-demand analytics job service for big data
 - you pay for the resources used by your query




