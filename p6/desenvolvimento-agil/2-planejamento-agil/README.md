# 2 Planejamento Ágil

## Visão do Produto e Escopo

Em projetos ágeis, o escopo não é um documento fechado, mas sim vivo. A principal forma de representar requisitos e funcionalidades desejadas é através do _Product Backlog_, alimentado por Histórias de Usuários.

## Product Backlog

Lista única e centralizada de tudo que o produto precisa ter.

- É de responsabilidade do **Product Owner (PO)** criá-la, mantê-la, priorizá-la (pelo valor de negócio) e refiná-la.
- Antes da _Sprint Planning_, o topo do backlog deve estar detalhado o suficiente (com critérios de aceite e estimativas) para ser puxado pela equipe.

## Histórias de Usuário (_User Stories_)

Formato curto e focado no valor para o cliente, em oposição a documentos de especificação extensos.

- **Sintaxe clássica:** "Como [ator], eu quero [ação] para [justificativa/valor]".
- **Critérios de Aceite:** Regras claras no "verso do cartão" que definem quando a história está cumprida (condições de teste).

### Princípio INVEST

Acrônimo para garantir a qualidade de uma História de Usuário:

- **I**ndependent (Independente): Pode ser entregue isoladamente.
- **N**egotiable (Negociável): É um convite à conversa, não um contrato fixo.
- **V**aluable (Valiosa): Entrega valor real para o negócio/cliente.
- **E**stimable (Estimável): A equipe consegue prever o esforço/complexidade.
- **S**mall (Pequena): Cabe dentro de uma _Sprint_.
- **T**estable (Testável): Possui critérios claros para ser validada.

## Temas, Épicos e Features

Quando histórias são muito grandes, elas recebem classificações hierárquicas:

- **Tema:** Agrupamento lógico de histórias por afinidade ou área de negócio (ex: "Módulo Financeiro").
- **Épico (_Epic_):** Uma história de usuário muito grande que não cabe em uma única _Sprint_. Deve ser fatiada em histórias menores.
- **Feature:** Funcionalidades sistêmicas que agrupam _User Stories_.

## O Fluxo do Planejamento

1. **Refinamento do Backlog:** PO e Equipe detalham e estimam histórias futuras.
2. **Sprint Planning:**
   - Define a **Meta da Sprint** (objetivo principal).
   - O PO apresenta os itens de maior prioridade.
   - A Equipe de Desenvolvimento decide _quantos_ itens consegue puxar para a _Sprint_ e _como_ vai construí-los (quebrando em tarefas técnicas).
   - O resultado é o **Sprint Backlog**.

## Estimativas e _Planning Poker_

O Scrum não estima em horas exatas, mas em **pontos de complexidade/esforço relativo**.

- **Planning Poker:** Técnica baseada na sequência de Fibonacci (1, 2, 3, 5, 8, 13, 21...).
- **Dinâmica:** A equipe lê a história, tira dúvidas com o PO e cada membro escolhe uma carta anonimamente. Revelam-se as cartas ao mesmo tempo para evitar viés (efeito manada). Divergências grandes geram debate técnico até o consenso. O objetivo principal é o alinhamento, não apenas o número final.
