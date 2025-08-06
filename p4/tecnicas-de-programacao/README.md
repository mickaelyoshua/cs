# Programming Techniques
## DevOps Terminology
DevOps is a software development aproach that mix precesses, technologies and people to a continuous deliver of values for the client.

## Main Practices and Tools for DevOps
* **Continuous Integration (CI):** Frequent integration of code from different developers;
* **COnituous Delivery (CD):** Frequent deliver of validated (tested and approved) code to the product;
* **Continuous Monitoring:** Monitor applications, infraestructures and logs;
* **Test Automation:** Creation of automated pipelines to execute differents test as unit, integration, regression etc;
* **Collaboration Culture:** Practice of different development teams to work together.

### Some Tools for DevOps
#### Gestão de Código e Repositórios 🗂️

| Ferramenta | Descrição |
| :--- | :--- |
| Git | Sistema de controle de versão distribuído. |
| Github | Plataforma de hospedagem de repositórios. |
| GitLab | Plataforma de hospedagem de repositórios. |
| Bitbucket | Plataforma de hospedagem de repositórios. |

---
#### CI/CD (Integração Contínua/Entrega Contínua) ⚙️

| Ferramenta | Descrição |
| :--- | :--- |
| Jenkins | Servidor de automação open-source. |
| GitHub Action | Plataforma de CI/CD integrada ao GitHub. |
| GitLab CI/CD | Plataforma de CI/CD integrada ao Gitlab. |
| Circle CI | Plataformas de CI/CD em nuvem. |
| Travis CI | Plataformas de CI/CD em nuvem. |
| Azure DevOps | Plataformas de CI/CD em nuvem. |

---
#### Conteinerização e Orquestração de contêineres 📦

| Ferramenta | Descrição |
| :--- | :--- |
| Docker | Plataforma de conteinerização. |
| Kubernets | Plataforma de orquestração de containers. |
| Openshift | Plataforma de orquestração de containers. |

---
#### Infraestrutura como Código (IaC) 🏗️

| Ferramenta | Descrição |
| :--- | :--- |
| Terraform | Ferramenta de provisionamento de infraestrutura. |

---
#### Monitoramento 📊

| Ferramenta | Descrição |
| :--- | :--- |
| Prometheus | Sistema de monitoramento e alertas. |
| Grafana | Sistema de visualização de dados (comumente utilizado junto ao Prometheus). |
| New Relic | Sistema de monitoramento e alertas. |
| Splunk | Plataformas de monitoramento e análise de dados. |
| Datadog | Plataformas de monitoramento e análise de dados. |

---
#### Cloud ☁️

| Ferramenta | Descrição |
| :--- | :--- |
| AWS | Plataformas de nuvem. |
| Microsoft Azure | Plataformas de nuvem. |
| Google Cloud | Plataformas de nuvem. |

---
#### Ferramentas de Segurança DevOps 🛡️

| Ferramenta | Descrição |
| :--- | :--- |
| SonarQube | Plataforma de análise de código estático. |
| Snyk | Ferramenta de gerenciamento de vulnerabilidades. |

## Main Performance Metrics
On DevOps there is a word called **DORA (DevOps Research and Assessment)**. It consists on four main metrics:
* Deployment Frequency;
* Lead Time for Changes;
* Change Failure Rate;
* Mean Time to Restore.


# Test Driven Development (TDD)
## What is TDD and Tests Pyramid
On TDD, intead of start the project on sotfware development and test after, the test are developed first, then the software itself.

There are different test levels on TDD:
* **Unit tests:** Check small parts of the code like functions, methods or conditions;
* **Integration tests:** Check how modules and components on a software integrate between thenselfs;
* **End to End test:** Simulate how the system should behave on a real scenario.

## TDD's Basic Principles
* **Red:** Initiate with a test that fails, not because if wrong, but because there is no code yet;
* **Green:** The code created pass on the tests developed on **Red**;
* **Refactor:** Improve the tests and the code created bafore.

# Automation and Documentation of APIs
## Swagger
Swagger is an open-source set of services that make the creation of documentation easy. In 2015, *Swagger Specification* was donated to the *OpenAI Initiative*.

The main components of Swagger are:
* **Swagger UI:** Interactive interface to create documentation of APIs that alows developers to test directly their application on the browser;
* **Swagger Editor:** On-line editor, like an IDE, that allows to generate yaml or json files giving automatic documentation generation;
* **Swagger Codegen:** A tool to generate code on multiple languages based on OpenAI specifications;
* **SwaggerHub:** Collaborative platform to manage and share specifications.

## Postman
Postman is a popular and versatile tool for developers that aids in the development, testing, and documentation of applications. It allows users to create various HTTP requests, such as GET, POST, PUT, and DELETE, to validate if applications are functioning correctly.

Key features include an intuitive, multi-platform interface (macOS, Windows, and Linux), and it can be used for free. Postman supports test automation with JavaScript, organization of requests into collections for different environments (like production or testing), and can even be used for API performance monitoring. Ultimately, it is a powerful tool that helps teams collaborate by sharing collections and improving testing and documentation.

## API Gateway
An API Gateway centralize the access permission to APIs and multiple functionalities like load balancing, data manipulation and rate limitation.

## Docker
Docker is a containerization platform that packages applications and their dependencies into portable units called containers. This ensures that applications run consistently across different machines and operating systems.

Unlike virtual machines that emulate an entire operating system and are very large, Docker containers are lightweight (occupying megabytes instead of gigabytes) because they only include the essential components needed to run the application.

For managing multi-container applications, a tool called Docker Compose is used to define and run the entire infrastructure from a single configuration file.

# Application Development
## What is CI/CD?
Continuous Integration (CI) and Continuous Deployment (CD) are the processes to integrate the codebase on a continuous and automatic whay minimizing human varification errors, ensuring more safity for the development process.

## CI Tools
* **Travis CI:** Integration tools hosted on the cloud. Used on open-source projects. Simplified configuration through yaml files to define the workflow;
* **Jenkins:** Open souce tool, can create hosted environments on the cloud or locally. Initial configuration is more complex but is highly utilized for be versatile and customizable;
* **GitLab CI/CD:** Tool integrated to GitLab. Simply to configure.

| Critério | Travis | Jenkins | GitLab CI/CD |
| :--- | :--- | :--- | :--- |
| **Hospedagem** | Cloud-hosted. | Cloud or on-premise. | GitLab-hosted. |
| **Setup Inicial** | Simple via `.travis.yml` file. | Complex; often needs DevOps team. | Simple via `.gitlab-ci.yml` file. |
| **Custos** | Free for open-source, paid for private. | Free software; hosting costs apply. | Included in GitLab plans. |
| **Linguagens** | Supports most languages. | Supports most languages via plugins. | Supports most languages via Docker. |
| **Plugins** | Limited plugin library. | Extensive library (>1800 plugins). | Limited to GitLab features. |
| **Interface** | Intuitive and simple UI. | Confusing UI, especially for beginners. | Intuitive and simple UI. |
| **Velocidade** | Fast execution. | Depends on local resources. | Scalable with runners. |
| **Containers** | Supports containers. | Requires plugins for containers. | Native Docker support. |
| **Comunidade** | Popular in open-source. | Large, mature community. | Growing DevOps community. |
| **Repositórios** | Mainly GitHub & Bitbucket. | Integrates with GitHub, Bitbucket, & GitLab. | Native GitLab integration. |

# Relational and Non-Relational Database
* **Relational Database:** Works with tables where each column is an attribute and each row is an individual register. Some of the main characteristics of relational databases are the consistency on the data structure and the capability to stablish a relation between tables through primary and foreign key;
* **Non-Relational Database:** Commonly knows as NoSQL, it can implement a variety of different models and structures:
    * Graphs: Representation on interlinked data allowing detailed analysis of relation between data;
    * Document Oriented: Data are stored on the JSON format where each document have its own unique key;
    * Key-Value: Data are stored on a structure of a key-value pair where the key is a unique identifier and the value is the data;
    * Column Model: Store data as columns. This optimize the search for big volumns of data.
