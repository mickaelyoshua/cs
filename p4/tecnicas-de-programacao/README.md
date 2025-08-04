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

