## Azure Core Solutions and Management Tools

https://docs.microsoft.com/en-us/learn/paths/az-900-describe-core-solutions-management-tools-azure/

management development tools for deploying and operating your app


### Choose the best Azure IoT ssrvice for your app

https://docs.microsoft.com/en-us/learn/modules/iot-fundamentals/1-introduction

 - devices with sensors -> cloud = IoT

#### Identify product options

 - smart device = sensor + internet -> send data for analysis (Azure endpoint)
 - update with new firmware from Azure
 - change device settings
 - use Azure AI to predict failures, inventory restock etc

Azure IoT Hub
 - central message hub between it and the devices
 - allows for command and control of the IoT devices from the cloud
 - offers a RESTful API

Azure IoT Central
 - on top of Hub - adds a dashbard for monitoring and managing your devices
 - easy UI to connect devices and monitor them
 - can set up alerts
 - solution templates for different industries
 - can push firmware updates, or modify a property on the device
 - create device templates for different type of devices
   (device developers must write code for the device to match)

Azure Sphere

 - security focused - end to end - hardware to cloud communication
 - dedicated hardware Seeed Azure Sphere MT3620, for example
 - customized Linux OS - communicates with AS3
 - Azure Sphere Security Service (AS3) - authenticates device software,
   sets up secure communication, can push new nedeveloped software

#### Analyze the decisiion criteria

Is it critical to ensure device not compromised?
 - Azure Sphere
 - ATM vs washing machine

Is a dashboard for reporting and management required?
 - only connect, telemetry, push some updates -> Hub
 - can create things on top using Hub RESTful API
 - prebuilt cutomizable user interface -> Central
 
#### Use IoT Hub

Example: send telemetry to see when maintenance should be scheduled. 
Software already exists that can manage maintenance requests

#### Use IoT Central

Example: full logistics monitoring and optimization (fleet prediction,
shipment integrity tracking)

"Connected Logistics" starter template

#### Use IoT Sphere

Example: touchless point-of-sale self checkout; plus reporting
-> Sphere + Central


### Choose the best AI service for your needs

https://docs.microsoft.com/en-us/learn/modules/ai-machine-learning-fundamentals/

#### Identify the product options

 - deep learning (neural nets to learn)
 - machine larning (pattern analysis, models)
 
Azure Machine Learning
 - data -> train and test models -> use it for predictions
 - from getting the data, cleaning it
 - offers data scientists complete control over design and training of an alg

Azure Cognitive Services
 - prebuild machine learning models for general tasks (e.g.: text -> emotional
   sentiment; visual -> face)
 - no datascience required; access the services programmatically in apps
 - Language, Speech, Vision; Devision

Azure Bot Service
 - platforms for creating bots that understand and reply to questions
 - specific use case, not like the previous two

#### Analyze the decisiion criteria

Virtual agent that interfaces with humans?
 - Azure Bot Service
 - Marketplace can offer no-code solutions - e.g.: QnA Maker

Understanc content and meaning of images, vidoe, audio or translate text?
 - Azure Cognitive Services

Predict user behavior or provide recommendations?
 - Azure Cognitive Services Personalizer service

Predict future outcomes based on historical private data?
 - Azure Machine Learning

Build an own model, for whatever different task?
 - Azure Machine Learning

#### Use Machine Learning for decision support systems

Examples: suggest products that could be added to shopping cart when buying
a certain product. Use historical sales data, product availability etc for
a complex decision -> Azure Machine Learning

#### Use Cognitive Services for data analysis

Example: offer e-commerce platform in a different language, or languages.
-> Azure Cognitive Services Trnaslator can do that, with multilanguage support

#### Use Bot services for interactive chat experiences

Example: virtual agent to handle vast majority of questions that the Customer
Service team gets. -> Azure Bot Service (using the information on the website's
FAQ page, plus thousands of chat conversations between shoppers and customer
service), can be tweaked to improve experience.


### Choose the best Azure serverless technology for your business scenario

https://docs.microsoft.com/en-us/learn/modules/serverless-fundamentals/

Serveless computing - write or visually design code of what should happen on a
given trigger - don't care where it runs

#### Identify the product options

Usually for back-end scenarios, processing messages between systems.

Azure Functions
 - programming function that responds to an event (http req, new message, a
   timer etc). C#, Python, js, typescript, java, powershell
 - automatic scaling, billing only for what is used
 - stateless environment; if state is required, can use an Azure storage
   account
 - Durable Functions allows for chaining functions and maintaining state
 - usually under a second work

Azure Logic Apps
 - low-code/no-code dev platform; focused on visual workflow design
 - triggers - connectors - actions (no code)
 - can create custom connector with code
 - pricing depends also on the type of connectors used, not only execution

#### Analyze the decisiion criteria

Need to perform orchestration between well-known APIs?
 - Azure Logic Apps (could be done in Functions, but research needed)

Execute custome algorithms or specialized data parsing and lookups?
 - Azure Functions (use the power of the programming language)

Existing automated tasks written in an imperative programming language?
 - Azure Functions (port it there)

Prefer visual (declarative) workflow?
 - Azure Logic Apps

### Use Azure Functions

Example: send a JSON message to a hub when a product is sold, update the
inventory. Code runs nightly that processes all the messages. 
-> Azure Functions (do this in real-time, using the existing code)

### Use Azure Logic Apps

Example: send customer satisfaction surverys randomly after purchase. Try to
reach out to low scores/negative sentiment - analyze free form text, send an
apology email, a coupon, follow-up email schedule. No developer time for this
-> Azure Logic Apps (existing connectors - sentiment analysis Azure Cognitive
Services connectors, send an email Outlook connector, request for follow-up
Dynamics365)


### Choose the best tools to help organizations build better solutions

https://docs.microsoft.com/en-us/learn/modules/azure-devops-devtest-labs/

 - devops, software development tools and processes
 - 


















